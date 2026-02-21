<!-- dprint-ignore-file -->
<!-- sync-readme title [[ -->
# cargo-sync-readme2
<!-- sync-readme ]] -->

<!-- sync-readme badge -->

<!-- sync-readme rustdoc [[ -->
A tool to sync your crate’s rustdoc documentation to your README.

This tool reads the rustdoc JSON output and renders it into markdown,
replacing marked sections in your README file.

### Usage

First, generate rustdoc JSON for your crate:

````bash
cargo +nightly rustdoc -- -Z unstable-options --output-format json
````

Then sync your README:

````bash
cargo sync-readme2 sync \
    --cargo-toml Cargo.toml \
    --rustdoc-json target/doc/your_crate.json \
    --readme-md README.md
````

Or test if it’s in sync (useful for CI):

````bash
cargo sync-readme2 test \
    --cargo-toml Cargo.toml \
    --rustdoc-json target/doc/your_crate.json \
    --readme-md README.md
````

For workspace-wide operations:

````bash
cargo sync-readme2 workspace sync
cargo sync-readme2 workspace test
````

### README Markers

Add markers to your README to indicate where content should be synced:

* `<!-- sync-readme title -->` - Inserts the crate name as an H1 heading
* `<!-- sync-readme badge -->` - Inserts configured badges
* `<!-- sync-readme rustdoc -->` - Inserts the crate’s rustdoc documentation
<!-- sync-readme ]] -->
