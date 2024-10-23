{ pkgs }: pkgs.rustPlatform.buildRustPackage {
  pname = "chat-app-server";
  src = ./.;
  version = "0.0.0";
  cargoLock.lockFile = ./Cargo.lock;
}
