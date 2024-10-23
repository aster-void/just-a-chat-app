# Just A Chat App -- Development

## 環境構築

1. Nix と Nix-direnv をインストールします。
2. リポジトリ直下で、 `direnv allow` を実行します。

## 開発環境のセットアップ

`just dev` ですべて起動します。
以下は、 `just dev` で実行される Just スクリプトのツリーです。

- dev
  - dev-db
    - `./runners/local-db.sh`
  - watch
    - watch-web
      - `cd web; bun run watch`
    - watch-server
      - `cd server; cargo watch --exec run`
