{
  description = "Just A Chat App";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = import ./nix/rust-toolchain.nix { inherit fenix system; };
        tauri = import ./nix/tauri.nix { inherit pkgs; };
        tools = import ./nix/tools.nix { inherit pkgs; };
        server = import ./server { inherit pkgs; };

        devShell = pkgs.mkShell {
          buildInputs = tools ++ tauri.packages ++ [ toolchain ];
          shellHook = tauri.shellHook + ''
            lefthook install
          '';
        };
      in
      {
        inherit devShell;
        packages = {
          default = server;
        };
      });
}
