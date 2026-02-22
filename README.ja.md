
# vsc-hue 🦀

`vsc-hue` は、Cursor や VS Code のワークスペースカラーを瞬時に変更する Rust 製の軽量 CLI ツールです。

## 🚀 1. インストール

Rust（cargo）がインストールされている環境で、以下のコマンドを実行してください。

### 方法A: リポジトリから直接インストール（推奨）
```bash
cargo install --git https://github.com/tkhrk1010/vsc-hue

```

### 方法B: クローンしてインストール

```bash
git clone https://github.com/tkhrk1010/vsc-hue
cd vsc-hue
cargo install --path .
```

## 🛠️ 2. パスの設定 (zshユーザー用)

もしインストール後に `vsc-hue` と打って「command not found」と出る場合は、以下のコマンドを**一度だけ**実行してください。これで `~/.zshrc` に設定が書き込まれ、パスが通ります。

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc

```

## 💡 3. 使い方

プロジェクトのルートディレクトリ（`.vscode` がある場所）で実行します。

### 対話モード（カラーパレット表示）

引数なしで実行すると、プレビュー付きのメニューが表示されます。

```bash
vsc-hue

```

### 色を直接指定

```bash
vsc-hue "#42b883"

```

## 🎯 5. 特徴

* **高速起動**: シングルバイナリで動作。
* **カラープレビュー**: ターミナル上で色付きの ■ を表示。
* **簡単リセット**: メニューからいつでも元の設定に戻せます。

## 📝 6. License

MIT License
