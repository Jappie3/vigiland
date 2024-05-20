{
  description = "Inhibit idle behaviour of a Wayland compositor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system: function nixpkgs.legacyPackages.${system});
  in {
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [rustfmt cargo];
      };
    });
    packages = forAllSystems (pkgs: rec {
      default = vigiland;
      vigiland = pkgs.rustPlatform.buildRustPackage {
        name = "vigiland";
        pname = "vigiland";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        doCheck = true;
        nativeBuildInputs = [];
        buildInputs = [];
      };
    });
  };
}
