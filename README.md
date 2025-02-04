# Git Helper

`git-helper` is a command-line utility written in Rust, designed to simplify the management of multiple Git repositories. With its powerful yet easy-to-use commands, you can efficiently add, remove, update, and inspect repositories in one place.

---

## Key Features

- **Repository Management**: Add and remove repositories with simple commands.
- **Batch Operations**: Fetch, pull, and check the status of all repositories simultaneously.
- **Detailed Status Reports**: Get comprehensive insights into the state of your repositories.
- **Remote Branch Handling**: Easily clone and manage remote branches locally.
- **Git Graph Visualization**: View commit graphs for better insights.

---

## Requirements

- **Rust**: Required to compile the utility.
- **Linux Environment**: Currently designed to work on Linux-based systems.
- **Git**: Must have Git installed and accessible.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/nbrandolino/git-helper.git
   ```
2. Navigate to the project directory:
   ```bash
   cd git-helper
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Install the tool:
   ```bash
   cargo install --path .
   ```

---

## Usage

Run the tool using the following command:
```bash
git-helper [OPTIONS]
```

### Available Options

- `-h, --help`: Display help information.
- `-V, --version`: Display version information.
- `-a, --add-repo <PATH>`: Adds a new repository to be managed.
- `-r, --remove-repo <IDENTIFIER>`: Removes a repository from management.
- `-l, --list-repos`: Lists all managed repositories.
- `-s, --status`: Displays a detailed status report for all managed repositories.
- `-p, --pull-all`: Pulls the latest changes for all managed repositories.
- `-f, --fetch-all`: Fetches updates for all managed repositories.
- `-c, --clone-remote-branches <IDENTIFIER>`: Creates local branches for all remote branches of a specific repository.
- `-g, --show-graph <IDENTIFIER>`: Displays the Git commit graph for the specified repository.

### Examples

1. Add a repository:
   ```bash
   git-helper -a /path/to/repo
   ```

2. Remove a repository:
   ```bash
   git-helper -r repo-name
   ```

3. List all repositories:
   ```bash
   git-helper -l
   ```

4. Display a status report:
   ```bash
   git-helper -s
   ```

5. Pull changes for all repositories:
   ```bash
   git-helper -p
   ```

6. Fetch updates for all repositories:
   ```bash
   git-helper -f
   ```

7. Clone remote branch:
   ```bash
   git-helper -c repo-name
   ```
8. Clone all remote branches:
   ```bash
   git-helper -c all
   ```
9. Show the commit graph:
   ```bash
   git-helper -g repo-name
   ```

---

## Configuration

The tool uses a configuration file located at:
```bash
~/.config/git-helper/git-helper.toml
```
This file stores the list of repositories being managed.

---



## License

This tool is licensed under the GNU General Public License (GPL). See the `LICENSE` file for more details.

---

## Contact

- **Author**: nbrandolino  
- **Email**: [nickbrandolino134@gmail.com](mailto:nickbrandolino134@gmail.com)
