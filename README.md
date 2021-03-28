# tree
CLI tool to view your directory tree. This is based on [Linux tree](https://linux.die.net/man/1/tree) with added features:
* Add a configuration file to automatically hide directories in the same format as `.gitignore` (such as hiding `target/`, `__pycache__/`) (.treeconfig)
* Option to ignore directories listed in the projects `.gitignore` (-g?)
