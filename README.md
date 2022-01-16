# tree
A CLI tool to list your directory tree.

This is a simplified implementation of UNIX [tree](https://linux.die.net/man/1/tree)
with added features for filtering files. By default will hide all files
listed in the current workspaces `.gitignore` (disabled with `-g` flag) as
its often frustrating to list output directories like `venv/` and `target/`.

## Getting Started

### Download
```sh
wget https://github.com/dunstall/tree/releases/download/a.b.c/tree-variant
chmod +x tree-variant
mv tree-variant /usr/local/bin/tree
```

### Installation
```sh
git clone git@github.com:dunstall/tree.git && cd tree
cargo build --release
cp target/release/tree /usr/local/bin
```

### Testing
```sh
git clone git@github.com:dunstall/tree.git && cd tree
cargo test
```

### Flags
See `tree --help` for the full CLI.

## Roadmap
* Add `-f` flag to filter files shown (`*.java`, `SB*.h*`, ...)
* Pretty print byte count (KB, MB...)
* Add glob support
* Add missing matching rules for `.gitignore` from `https://git-scm.com/docs/gitignore#_pattern_format`
* Add support for multiple directories (`tree bin/ src/`)

## Licence
See `LICENSE` for more information.
