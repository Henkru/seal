{
  description = "Seal your secrets";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { flake-parts, rust-overlay, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        rec {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
            ];
          };

          devShells.default =
            with pkgs;
            mkShell {
              buildInputs = [
                rust-bin.stable.latest.default
                rust-analyzer
                clippy
                rage
                age-plugin-yubikey
              ];
            };

          packages.seal =
            let
              deps = with pkgs; [
                rage
                age-plugin-yubikey
              ];
              app = pkgs.rustPlatform.buildRustPackage {
                pname = "seal";
                version = "0.2.0";
                cargoLock.lockFile = ./Cargo.lock;
                src = pkgs.lib.cleanSource ./.;

                meta = {
                  description = "Encrypt secrets with age and generate corresponding QR code";
                  homepage = "https://github.com/Henkru/seal";
                  license = pkgs.lib.licenses.mit;
                  maintainers = [ ];
                };
              };
            in
            pkgs.symlinkJoin {
              name = app.name;
              paths = [ app ] ++ deps;
              buildInputs = [ pkgs.makeWrapper ];
              postBuild = "wrapProgram $out/bin/seal --prefix PATH : $out/bin";
            };

          packages.default = packages.seal;
        };
    };
}
