{
    inputs = {
        naersk.url = "github:nix-community/naersk/master";
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        utils.url = "github:numtide/flake-utils";
    };

    outputs = {
        naersk,
        nixpkgs,
        utils,
        ...
    }:
        utils.lib.eachDefaultSystem (
            system: let
                pkgs = import nixpkgs {inherit system;};
                naersk-lib = pkgs.callPackage naersk {};
            in {
                packages.default = naersk-lib.buildPackage ./.;
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        cargo
                        rustc
                        rustfmt
                        pre-commit
                        rustPackages.clippy
                    ];
                    RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
                };
            }
        );
}
