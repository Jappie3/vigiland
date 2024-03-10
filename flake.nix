{
  description = "Inhibit idle behaviour of a Wayland compositor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        pkgs,
        system,
        self',
        ...
      }: {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [rustfmt cargo];
        };
        packages = {
          default = self'.packages.wayfreeze;
          wayfreeze = pkgs.rustPlatform.buildRustPackage {
            name = "Vigiland";
            pname = "Vigiland";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = true;
            nativeBuildInputs = [];
            buildInputs = [];
          };
        };
      };
    };
}
