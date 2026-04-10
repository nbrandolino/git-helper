# Git Helper
`git-helper` is a command-line utility written in Rust, designed to simplify the management of multiple Git repositories. It can efficiently add, remove, update, and inspect repositories in one place.

## Requirements
- **Rust**: Required to compile the utility.
- **Linux Environment**: Currently designed to work on Linux-based systems.
- **Git**: Must have Git installed and accessible.

## Installation
### Install From Source
1. Navigate to the project directory:
   ```bash
   cd git-helper
   ```
2. Install:
   ```bash
   cargo install --path .
   ```

## Configuration
The configuration file is located at:
```
~/.config/git-helper/git-helper.toml
```
This file stores the list of repositories being managed.

## Usage
```bash
git-helper [OPTIONS]
```

---

## Options

| Option | Description |
|---|---|
| `-h, --help` | Display help information |
| `-V, --version` | Display version information |
| `-C, --config <PATH>` | Specify an alternative configuration file |
| `-q, --quiet` | Suppress informational output; only show warnings and errors |
| `-a, --add-repo <PATH>` | Add a repository to be managed |
| `-r, --remove-repo <IDENTIFIER>` | Remove a repository by path or name |
| `-l, --list-repos` | List all managed repositories |
| `-d, --detect-repos <DIRECTORY>` | Detect Git repositories in the immediate children of a directory and add them to the configuration |
| `-p, --pull` | Pull the latest changes for all managed repositories |
| `-P, --push` | Push the latest changes for all managed repositories to their remotes |
| `-c, --clone-remote-branches <IDENTIFIER>` | Create local branches for all remote branches of a specific repository |

---

### add-repo
```bash
git-helper -a /path/to/repo
```

### remove-repo
Accepts either a full path or a repository name.
```bash
git-helper -r /path/to/repo
git-helper -r repo-name
```

### detect-repos
```bash
git-helper -d /path/to/directory
git-helper -d .
```

### pull
```bash
git-helper -p
git-helper -p -q    # suppress output
```

### push
```bash
git-helper -P
git-helper -P -q    # suppress output
```

### clone-remote-branches
Creates a local tracking branch for every remote branch in the specified repository. Accepts a name or `all` to run against every managed repository.
```bash
git-helper -c repo-name
git-helper -c all
```

---

## Disclaimer
> **Note:** This GitHub repository is a mirror of a private, self-hosted GitLab repository.

## License
This tool is licensed under the GNU General Public License (GPL). See the `LICENSE` file for more details.

## Contact
- **Author**: nbrandolino
- **Email**: [nickbrandolino134@gmail.com](mailto:nickbrandolino134@gmail.com)
