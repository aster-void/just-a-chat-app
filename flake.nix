{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        tauri = import ./nix/tauri.nix { inherit pkgs; };
        tools = import ./nix/tools.nix pkgs;

        tauri-client = pkgs.stdenv.mkDerivation {
          name = "just-a-chat-app:tauri-client";
          buildInputs = tauri.packages ++ (with pkgs; [ bun ]);
          shellHook = tauri.shellHook;
          buildPhase = ''''; # todo
        };

        server = pkgs.rustPlatform.buildRustPackage {
          # todo
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # compiler + runtime + package manager
            bun
            rustc
            cargo

            # dev tools
            just
            sqlx-cli
            cargo-watch
            # bacon
          ] ++ tauri.packages ++ tools;
          shellHook = tauri.shellHook;
        };
      in
      {
        devShell = devShell;
        packages = {
          inherit tauri-client server;
          default = tauri-client;
        };
      });
}
