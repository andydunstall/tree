# tree
A CLI tool to view your directory tree.

This is based on [Linux tree](https://linux.die.net/man/1/tree) with added
features to confgiure files to ignore and hopefully more extendible.

## Getting Started

### Download
  ```sh
  wget https://github.com/dunstall/tree/releases/download/a.b.c/tree
  mv tree /usr/local/bin
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

### Configuration
The purpose of this `tree` implementation is to allow easily hiding
directories and files, such as `venv/` and `__pycache__` in Python, `target` in
Rust. There are 3 ways to support this:
* `-I` CLI option to specify a path to ignore,
* '-g' option to ignore files listed in the workspaces `.gitignore`,
* ignores files listed in `~/.treeignore` by default, which is the same format
as `.gitignore` (with a few exceptions listed below). This can be disabled
using the `-c` option in the CLI.

### .treeignore
`~/.treeignore` is in the same format as `.gitignore` (see [format](https://git-scm.com/docs/gitignore#_pattern_format)),
with exceptions:
* Patterns with a leading slash `/` will be seen as absolute paths rather than
relative to the `.treeignore` file (unlike in `.gitignore` where it is relative
to the `.gitignore` file itself.

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
