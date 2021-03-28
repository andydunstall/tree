# tree
A CLI tool to view your directory tree.

This is based on [Linux tree](https://linux.die.net/man/1/tree) with added
features to confgiure files to ignore and hopefully more extendible.

## Getting Started

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

## Roadmap
* v0.2.0:
  * ignore files listed in `~/.treeignore` by default (with a `-c` option to
ignore this) in the same format as `.gitignore`,
  * `-g` option to ignore files listed `.gitignore` (including all excludes
upto the workspace root),
  * `-I` option to ignore files using the same glob format as above.
* v0.3.0:
  * don't follow symlinks by default and add a `-l` option to follow,
  * support listing multiple root directories not just one.

## Licence
See `LICENSE` for more information.
