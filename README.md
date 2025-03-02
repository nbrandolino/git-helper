# Git Helper
`git-helper` is a command-line utility written in Rust, designed to simplify the management of multiple Git repositories. It can efficiently add, remove, update, and inspect repositories in one place.

## Requirements
- **Rust**: Required to compile the utility.
- **Linux Environment**: Currently designed to work on Linux-based systems.
- **Git**: Must have Git installed and accessible.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/nbrandolino/git-helper.git
   ```
2. Navigate to the project directory:
   ```bash
   cd git-helper
   ```
3. Install the tool:
   ```bash
   cargo install --path .
   ```

## Usage
Run the tool using the following command:
```bash
git-helper [OPTIONS]
```

### Available Options
- `-h, --help`: Display help information.
- `-V, --version`: Display version information.
- `-C, --config <PATH>`: Specify an alternative configuration file.
- `-a, --add-repo <PATH>`: Adds a new repository to be managed.
- `-r, --remove-repo <IDENTIFIER>`: Removes a repository from management.
- `-l, --list-repos`: Lists all managed repositories.
- `-d, --detect-repos`: Detects Git repositories in the specified directory and adds them to the configuration.
- `-p, --pull-all`: Pulls the latest changes for all managed repositories.
- `-P, --push-all`: Pushes the latest changes for all managed repositories to their remotes.
- `-f, --fetch-all`: Fetches updates for all managed repositories.
- `-c, --clone-remote-branches <IDENTIFIER>`: Creates local branches for all remote branches of a specific repository.

### Examples
1. Specify configuration file:
   ```bash
   git-helper -C /path/git-helper.toml
   ```
2. Add a repository:
   ```bash
   git-helper -a /path/to/repo
   ```
3. Remove a repository:
   ```bash
   git-helper -r repo-name
   ```
4. List all repositories:
   ```bash
   git-helper -l
   ```
5. Detect new repositories in given directory:
   ```bash
   git-helper -d .
   ```
6. Pull changes for all repositories:
   ```bash
   git-helper -p
   ```
7. Push changes for all repositories:
   ```bash
   git-helper -P
   ```
8. Fetch updates for all repositories:
   ```bash
   git-helper -f
   ```
9. Clone remote branches:
   ```bash
   git-helper -c repo-name
   ```

## Configuration
The tool uses a configuration file located at:
```bash
~/.config/git-helper/git-helper.toml
```
This file stores the list of repositories being managed.

## License
This tool is licensed under the GNU General Public License (GPL). See the `LICENSE` file for more details.

## Contact

- **Author**: nbrandolino
- **Email**: [nickbrandolino134@gmail.com](mailto:nickbrandolino134@gmail.com)
