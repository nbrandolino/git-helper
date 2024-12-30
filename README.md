# Git Helper

## About

Written in C, `git-helper` is a command-line utility that utilizes the git command to help and automate various actions (pulling repos, etc.). The tool requires the configuration of a .conf file (`~/.config/git-helper/git-helper.conf`).

## Prerequisites

This tool requires the following to run:

- Linux-based operating system.
- GCC.
- Git.

Compile and Install
-------------------
```bash
$ cd ./git-helper
$ make
$ make install
```

Usage
-----
```bash
$ git-helper [options]
```

Help
---------------------
    Options:
        -h, --help                  Display this help message.
        -v, --version               Display version information.

    Example:
        $ git-helper -mp

Uninstall
---------
```bash
$ cd ./git-helper
$ make uninstall
```

## Additional Information
- The make file is written to allow for static compiling of the program.
    - Statically compile by `$ make static`

## License

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

    See './LICENSE' for more information.


## Original author

Nick Brandolino aka nbrandolino
nickbrandolino134@gmail.com
