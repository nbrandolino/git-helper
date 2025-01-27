# git-helper
`git-helper` is a command-line utility written in rust that allows for easy management of many git repositories.

## Requirements
- **Rust**: The tool is written in Rust and it is required to compile.
- **Linux Environment**: Designed to work on Linux-based systems.
- **Git**: Utilizes the Git package and requires it to run.

## Usage
```bash
git-helper [options]
```

### Options:
- `-h, --help`: Display help information.
- `-V, --version`: Display version information.
- `-a, --add-repo`: Adds a new repository to be managed.
- `-r, --remove-repo`: Removes a repository from being managed.
- `-l, --list-repos`: Lists all repositories being managed.
- `-p, --pull-all`: Pulls all managed repositories.
- `-f, --fetch-all`: Fetches all managed repositories.
- `-c, --clone-remote-branches`: Creates local branches for all remote branches of a specified repository.

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
5. Fetch all repositories:
    ```bash
    git-helper -f
    ```
6. Create local branches for all remote branches:
    ```bash
    git-helper -c repo-name
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

## Additional Information
- The program creates and saves a config file located at `~/.config/git-helper/git-helper.toml`

## License
This tool is licensed under the GNU General Public License (GPL). See ./LICENSE for more details.

## Contact
nbrandolino
nickbrandolino134@gmail.com
