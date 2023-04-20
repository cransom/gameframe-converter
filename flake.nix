{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };


  outputs = inputs@{ flake-parts, crane, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule

      ];
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
      ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          craneLib = crane.lib.${system};
          gameframe-converter = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            buildInputs = [
              # Add additional build inputs here
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

            # Additional environment variables can be set directly
            # MY_CUSTOM_VAR = "some value";

          };
          crun = pkgs.writeShellApplication {
            name = "crun";
            runtimeInputs = [ pkgs.entr ];
            text = ''
              find src/ -iname \*.rs | entr -r cargo run -- "$*"
            '';

          };
        in
        rec
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.

          # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
          checks = { inherit gameframe-converter; };
          packages.default = gameframe-converter;
          apps.default = {
            type = "app";
            program = gameframe-converter + "/bin/" + gameframe-converter.pname;
          };
          devShells.default = pkgs.mkShell {
            name = "gameframe-converter";
            #inputsFrom = gameframe-converter.buildInputs;
            inputsFrom = builtins.attrValues checks ++ [
            ];

            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            nativeBuildInputs = with pkgs; [
              # macos rustfmt is broken and i'm sad about it.
              # cargo rustc rustfmt
              rustup
              crun
              coreutils
            ];
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
