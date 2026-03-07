{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    alejandra = {
      url = "github:kamadorueda/alejandra/4.0.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    alejandra,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        llvmPackages = pkgs.llvmPackages_21;
      in {
        devShells.default = with pkgs;
          mkShell {
            name = "crates-shell";

            buildInputs = [
              git
              rustup
              just
              dprint
              buf
              shfmt
              cargo-deny
              cargo-insta
              cargo-nextest
              cargo-hakari
              cargo-llvm-cov
              cargo-semver-checks
              protobuf_33
              sccache
              mold
              llvmPackages.clang
              miniserve
              release-plz
              lcov
            ];

            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";

            shellHook = ''
              mkdir -p "$PWD/.cache"
              export SCCACHE_SERVER_UDS="$PWD/.cache/sccache.sock"
            '';
          };

        formatter = alejandra.defaultPackage.${system};
      }
    );
}
