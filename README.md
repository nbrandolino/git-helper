# git-helper
`git-helper` is a command-line utility written in rust that allows for easy management of many git repositories.

## Requirements
- **Rust**: The tool is written in Rust and it is required to compile.
- **Linux Environment**: Designed to work on Linux-based systems.

## Usage
```bash
git-helper [options]
```

### Options:
- `-h, --help`: Display help information.
- `-v, --version`: Display version information.
- `-a, --add-repo`: Adds a new repository to be managed.
- `-r, --remove-repo`: Removes a repository from being managed.
- `-l, --list-repos`: Lists all repositories being managed.
- `-p, --pull-all`: Pulls all managed repositories.

### Examples
1. Add repository:
    ```bash
    git-helper -a /path/to/repo
    ```
2. Remove repository:
    ```bash
    git-helper -r repo-name
    ```
3. List all repositories:
    ```bash
    git-helper -l
    ```
4. Pull all repositories:
    ```bash
    git-helper -p
    ```

## Build and Install
To build the project, ensure you have `rust` installed, then compile the code as follows:

```bash
cd ./git-helper
```
```bash
cargo build --release
```
```bash
cargo install --path .
```

## License
This tool is licensed under the GNU General Public License (GPL). See ./LICENSE for more details.

## Contact
nbrandolino
nickbrandolino134@gmail.com
