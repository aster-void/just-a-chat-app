{ pkgs }: pkgs.rustPlatform.buildRustPackage {
  pname = "server";
  src = ./.;
  version = "0.0.0";
  cargoLock.lockFile = ./Cargo.lock;
}
