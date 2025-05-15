{
    pkgs ? import ./locked.nix,
    input,
}: let
    inherit (pkgs) lib;
    inherit (lib.strings) splitString toUpper;
    inherit (lib.lists) imap0;
    inherit (builtins) readFile filter match elem;

    instructions = [
        "nop"
        "input"
        "output"
        "jmp"
        "load"
        "inc"
        "mov"
        "add"
        "halt"
    ];

    strContains = str: substr: elem substr (splitString "" str);
in
    input
    |> readFile
    |> splitString "\n"
    |> filter (line: line != "")
    # handle each line
    |> imap0 (
        i: line:
            line
            |> splitString " "
            |> filter (word: word != "")
            |> map toUpper
            |> map (word:
                if strContains word ":"
                then word
                else "${word}:")
    )
