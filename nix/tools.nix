{ pkgs }: with pkgs; [
  # TypeScript
  bun
  biome

  # Rust
  clippy
  sqlx-cli
  cargo-watch
  # bacon

  # dev tools
  just
  nushell
]
