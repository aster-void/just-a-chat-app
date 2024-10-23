{ system, fenix }:
fenix.packages.${system}.fromToolchainFile {
  file = ../rust-toolchain.toml;
  sha256 = "sha256-3jVIIf5XPnUU1CRaTyAiO0XHVbJl12MSx3eucTXCjtE=";
}
