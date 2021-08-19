# tree
A CLI tool to view your directory tree.

This is a simplified simplementation of UNIX [tree](https://linux.die.net/man/1/tree) with added
support for hiding files listed in the current spaces `.gitignore` (using the `-g` flag). Such
as its often frustrating to get all of `venv/` in Python or `target/` in Rust.

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

### Configuration
The aim of this `tree` implementation is to support hiding directories and
files easily which would otherwise clutter the output, such as `venv/` and
`__pycache__` in Python, `target` in Rust. There are 3 supported ways to
hide files and directories:
* `-I` CLI option to specify a path to ignore,
* '-g' option to ignore files listed in the workspaces `.gitignore`,
* ignores files listed in `~/.treeignore` by default, which is the same format
as `.gitignore` (with a few exceptions listed below). This can be disabled
using the `-c` option in the CLI.

Note files beginning with `.` are hidden by default (which can be disabled
with `-a`).

See `tree --help` for the full CLI.

### .treeignore
`~/.treeignore` is in the same format as `.gitignore` (see [format](https://git-scm.com/docs/gitignore#_pattern_format)),
with exceptions:
* Patterns with a leading slash `/` will be seen as absolute paths rather than
relative to the `.treeignore` file (unlike in `.gitignore` where it is relative
to the `.gitignore` file itself.

## Roadmap
* v0.3.0:
  * don't follow symlinks by default and add a `-l` option to follow,
  * support listing multiple root directories not just one.
  * support ignored glob

### Know Issues
* Glob ignore not supported,
* Absolute ignore paths are taken as relative (ie removes leading `/`).

## Licence
See `LICENSE` for more information.
