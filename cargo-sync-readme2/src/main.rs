//! A tool to sync your crate's rustdoc documentation to your README.
//!
//! This tool reads the rustdoc JSON output and renders it into markdown,
//! replacing marked sections in your README file.
//!
//! ## Usage
//!
//! First, generate rustdoc JSON for your crate:
//!
//! ```bash
//! cargo +nightly rustdoc -- -Z unstable-options --output-format json
//! ```
//!
//! Then sync your README:
//!
//! ```bash
//! cargo sync-readme2 sync \
//!     --cargo-toml Cargo.toml \
//!     --rustdoc-json target/doc/your_crate.json \
//!     --readme-md README.md
//! ```
//!
//! Or test if it's in sync (useful for CI):
//!
//! ```bash
//! cargo sync-readme2 test \
//!     --cargo-toml Cargo.toml \
//!     --rustdoc-json target/doc/your_crate.json \
//!     --readme-md README.md
//! ```
//!
//! For workspace-wide operations:
//!
//! ```bash
//! cargo sync-readme2 workspace sync
//! cargo sync-readme2 workspace test
//! ```
//!
//! ## README Markers
//!
//! Add markers to your README to indicate where content should be synced:
//!
//! - `<!-- sync-readme title -->` - Inserts the crate name as an H1 heading
//! - `<!-- sync-readme badge -->` - Inserts configured badges
//! - `<!-- sync-readme rustdoc -->` - Inserts the crate's rustdoc documentation

use std::fmt;
use std::process::Command as ProcessCommand;

use anyhow::{Context, bail};
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Parser, Subcommand};
use console::{Style, style};
use similar::{ChangeTag, TextDiff};

use crate::config::{Metadata, Package};

mod config;
mod content;
mod render;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Sync(SyncArgs),
    Test(TestArgs),
    Workspace(WorkspaceArgs),
}

#[derive(Parser, Debug)]
struct SyncArgs {
    #[arg(long)]
    cargo_toml: Utf8PathBuf,

    #[arg(long)]
    rustdoc_json: Utf8PathBuf,

    #[arg(long)]
    readme_md: Utf8PathBuf,
}

#[derive(Parser, Debug)]
struct TestArgs {
    #[arg(long)]
    cargo_toml: Utf8PathBuf,

    #[arg(long)]
    rustdoc_json: Utf8PathBuf,

    #[arg(long)]
    readme_md: Utf8PathBuf,
}

#[derive(Parser, Debug)]
struct WorkspaceArgs {
    #[command(subcommand)]
    command: WorkspaceCommand,

    #[arg(long, default_value = "target/doc")]
    target_dir: Utf8PathBuf,
}

#[derive(Subcommand, Debug)]
enum WorkspaceCommand {
    Sync,
    Test,
}

#[derive(Clone, Default, serde_derive::Deserialize)]
struct ManifestMetadata {
    #[serde(alias = "sync-readme")]
    sync_readme: Metadata,
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Command::Sync(args) => sync(args),
        Command::Test(args) => test(args),
        Command::Workspace(args) => workspace(args),
    };

    if let Err(e) = result {
        eprintln!("error: {:#}", e);
        std::process::exit(1);
    }
}

fn load_package(cargo_toml: &Utf8PathBuf, rustdoc_json: Utf8PathBuf) -> anyhow::Result<Package> {
    let cargo_toml =
        cargo_toml::Manifest::<ManifestMetadata>::from_path_with_metadata(cargo_toml).context("cargo toml")?;
    let pkg = cargo_toml.package();

    Ok(Package {
        name: pkg.name.clone(),
        license: pkg.license().map(|l| l.to_owned()),
        version: pkg.version().to_owned(),
        rustdoc_json,
        metadata: pkg.metadata.clone().unwrap_or_default().sync_readme,
    })
}

fn render_readme(package: &Package, readme_path: &Utf8Path) -> anyhow::Result<(String, String)> {
    let content = content::create(package).context("content")?;
    let readme = std::fs::read_to_string(readme_path).context("readme read")?;
    let rendered = render::render(&readme, &content).context("render")?;
    Ok((readme, rendered))
}

fn sync(args: SyncArgs) -> anyhow::Result<()> {
    let package = load_package(&args.cargo_toml, args.rustdoc_json)?;
    let (_, rendered) = render_readme(&package, &args.readme_md).with_context(|| args.readme_md.to_string())?;
    std::fs::write(&args.readme_md, rendered).context("write readme")?;
    println!("synced {}", args.readme_md);
    Ok(())
}

fn test(args: TestArgs) -> anyhow::Result<()> {
    let package = load_package(&args.cargo_toml, args.rustdoc_json)?;
    test_package(&package, &args.readme_md)
}

fn test_package(package: &Package, readme_path: &Utf8Path) -> anyhow::Result<()> {
    let (source, rendered) = render_readme(package, readme_path).with_context(|| readme_path.to_string())?;

    if rendered == source {
        println!("readme matches render: {}", readme_path);
        return Ok(());
    }

    println!("Difference found in {}", readme_path);
    println!("{}", diff(&source, &rendered));
    bail!("readme out of sync: {}", readme_path)
}

struct WorkspacePackage {
    name: String,
    manifest_path: Utf8PathBuf,
    readme_path: Utf8PathBuf,
    metadata: Metadata,
}

fn get_workspace_packages() -> anyhow::Result<Vec<WorkspacePackage>> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("failed to get cargo metadata")?;

    let packages = metadata
        .packages
        .into_iter()
        .filter_map(|p| {
            Some(Ok(WorkspacePackage {
                name: p.name.to_string(),
                manifest_path: p.manifest_path,
                readme_path: p.readme?,
                metadata: if !p.metadata.is_null() {
                    match serde_json::from_value::<ManifestMetadata>(p.metadata) {
                        Ok(metadata) => metadata.sync_readme,
                        Err(e) => {
                            return Some(Err(e));
                        }
                    }
                } else {
                    Default::default()
                },
            }))
        })
        .collect::<Result<Vec<WorkspacePackage>, serde_json::Error>>()?;

    Ok(packages)
}

fn build_rustdoc_json(packages: &[WorkspacePackage], target_dir: &Utf8Path) -> anyhow::Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    println!("building rustdoc json for {}", packages.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", "));

    let mut cmd = ProcessCommand::new("cargo");
    cmd.env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .arg("doc")
        .arg("--no-deps")
        .arg("--target-dir").arg(target_dir);

    let mut features = Vec::new();

    for pkg in packages {
        cmd.args(["-p", &pkg.name]);
        features.extend(pkg.metadata.features.iter().map(|f| format!("{}/{f}", pkg.name)));
    }

    if !features.is_empty() {
        cmd.arg("--features").arg(features.join(","));
    }

    let status = cmd.status().context("failed to run cargo doc")?;

    if !status.success() {
        bail!("cargo doc failed");
    }

    Ok(())
}

fn workspace(args: WorkspaceArgs) -> anyhow::Result<()> {
    let packages = get_workspace_packages()?;

    if packages.is_empty() {
        println!("No packages with sync-readme metadata found");
        return Ok(());
    }

    build_rustdoc_json(&packages, &args.target_dir)?;

    let mut errors = Vec::new();

    for pkg in &packages {
        let json_name = pkg.name.replace('-', "_");
        let rustdoc_json = args.target_dir.join("doc").join(format!("{}.json", json_name));
        let package = load_package(&pkg.manifest_path, rustdoc_json)?;
        let readme_path = pkg.manifest_path.parent().unwrap().join(&pkg.readme_path);

        match args.command {
            WorkspaceCommand::Sync => {
                sync_package(&package, &readme_path)?;
            }
            WorkspaceCommand::Test => {
                if let Err(e) = test_package(&package, &readme_path) {
                    errors.push(e);
                }
            }
        }
    }

    if !errors.is_empty() {
        bail!("{} package(s) have out-of-sync READMEs", errors.len());
    }

    Ok(())
}

fn sync_package(package: &Package, readme_path: &Utf8Path) -> anyhow::Result<()> {
    let readme_path_buf = readme_path.to_path_buf();
    let (_, rendered) = render_readme(package, &readme_path_buf).with_context(|| readme_path.to_string())?;
    let original = std::fs::read_to_string(readme_path).context("read original readme")?;
    if original == rendered {
        println!("readme is already in sync: {}", readme_path);
        return Ok(());
    }

    std::fs::write(readme_path, rendered).context("write readme")?;
    println!("synced {}", readme_path);
    Ok(())
}

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

fn diff(old: &str, new: &str) -> String {
    use std::fmt::Write;

    let diff = TextDiff::from_lines(old, new);
    let mut output = String::new();

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            writeln!(&mut output, "{0:─^1$}┼{0:─^2$}", "─", 9, 120).unwrap();
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                write!(
                    &mut output,
                    "{}{} │{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                )
                .unwrap();
                for (_, value) in change.iter_strings_lossy() {
                    write!(&mut output, "{}", s.apply_to(value)).unwrap();
                }
                if change.missing_newline() {
                    writeln!(&mut output).unwrap();
                }
            }
        }
    }

    output
}

