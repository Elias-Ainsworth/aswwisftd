{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    { nixpkgs, systems, ... }:
    let
      forEachSystem =
        function: nixpkgs.lib.genAttrs (import systems) (system: function nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            # helper shell script
            (writeShellScriptBin "wtftd" ''cargo run -q -- "$@"'')
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
          ];
        };
      });

      packages = forEachSystem (pkgs: rec {
        aswwisftd = pkgs.rustPlatform.buildRustPackage {
          pname = "wtftd";
          version = "0.0.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "another fucking todo cli/tui(wip) that reminds you what the fuck to do.";
            license = licenses.mit;
            maintainers = [ lib.maintainers.elias-ainsworth ];
            platforms = platforms.all;
          };
        };
        default = aswwisftd;
      });
    };
}
