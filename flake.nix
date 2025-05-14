{
    description = "A simple assembler";
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    outputs = inputs: let
        pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
    in {
        packages.x86_64-linux.default = pkgs.writeShellApplication rec {
            name = "assembler";
            runtimeInputs = [pkgs.bash];

            text = ''
                input=""
                output=""

                usage() {
                    echo "Usage: ${name} --input FILE [--output FILE]"
                    exit 1
                }

                while [[ $# -gt 0 ]]; do
                    case "$1" in
                        --input)
                        input=$(realpath "$2")
                        shift 2
                        ;;
                        --output)
                        output=$(realpath "$2")
                        shift 2
                        ;;
                        *)
                        echo "Unknown argument: $1"
                        usage
                        ;;
                    esac
                done

                if [[ -z "$input" || -z "$output" ]]; then
                    echo "Missing required --input or --output argument."
                    usage
                fi

                if [[ ! -f "$input" ]]; then
                    echo "Error: Input file not found: $input"
                    exit 1
                fi

                nix eval --impure --expr "let parser = import ./parser.nix {input = \"$input\";}; in parser" "''${POSITIONAL[@]}"
            '';
        };
    };
}
