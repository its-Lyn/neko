{
  description = "neko";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    rust.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
      ];

      flake = {
        overlays.default = final: _prev: {
          neko = final.callPackage ./nix/package.nix { };
        };
      };

      perSystem = { config, pkgs, self', system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import inputs.rust)
            inputs.self.overlays.default
          ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            sudo
            pacman
            rust-bin.stable.latest.default
          ];
        };

        packages = {
          default = pkgs.neko;
          neko = self'.packages.default;
        };

        apps = {
          default.program = "${pkgs.neko}/bin/neko";
          neko = config.apps.default;
        };

        formatter = pkgs.nixpkgs-fmt;
      };
    };
}
