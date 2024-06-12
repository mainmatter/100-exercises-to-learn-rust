{
  description = "changeme";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # rustPlatform = pkgs.makeRustPlatform {
        #   cargo = pkgs.rust-bin.stable.latest.default;
        #   rustc = pkgs.rust-bin.stable.latest.default;
        # };

        # rustPackage = rustPlatform.buildRustPackage {
        #   name = "changeme";
        #   src = ./.;
        #   cargoLock.lockFile = ./Cargo.lock;
        # };

        # dockerImage = pkgs.dockerTools.buildImage {
        #   name = "changeme";
        #   config = {
        #     Cmd = [ "${rustPackage}/bin/changeme" ];
        #   };
        # };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            rustfmt
            pre-commit
            rustPackages.clippy
            # just
          ];
          shellHook = ''
            cat <<EOF
            Welcome to the \`changeme\` app development shell.
            EOF
            user_shell=$(getent passwd "$(whoami)" |cut -d: -f 7)
            exec "$user_shell"
          '';
        };

        # packages = {
        #   rustPackage = rustPackage;
        #   dockerImage = dockerImage;
        # };
      }
    );
}
