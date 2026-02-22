# vsc-hue 🦀

vsc-hue is a lightweight Rust CLI tool for instantly changing Cursor and VS Code workspace colors.

## 🚀 1. Installation
Install using cargo in a Rust environment.

### Method A: Direct installation (Recommended)

```bash
cargo install --git https://github.com/tkhrk1010/vsc-hue

```

### Method B: Clone and install

```bash
git clone https://github.com/tkhrk1010/vsc-hue
cd vsc-hue
cargo install --path .

```

## 🛠️ 2. Path Configuration (for zsh users)
If the command is not found after installation, run the following once to add the cargo bin path to ~/.zshrc.

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc

```

## 💡 3. Usage
Run the command in the project root directory where .vscode is located.

### Interactive Mode (Color Palette)
Run without arguments to display a menu with previews.

```bash
vsc-hue

```

### Direct Color Specification

```bash
vsc-hue "#42b883"

```

## 🎯 5. Features

* Fast startup: Operates as a single binary.
* Color preview: Displays colored icons in the terminal.
* Easy reset: Revert to original settings anytime through the menu.

## 📝 6. License

MIT License
