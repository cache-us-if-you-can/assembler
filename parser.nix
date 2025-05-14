{
    pkgs ? import ./locked.nix,
    input,
}: let
    inherit (pkgs) lib;
    inherit (lib.strings) splitString;
    inherit
        (builtins)
        readFile
        filter
        ;
in
    input
    |> readFile
    |> splitString "\n"
    |> filter (line: line != "")
