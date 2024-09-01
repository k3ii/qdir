# qdir - Quick Directory Creator
`qdir` is a command-line tool for creating directories with randomly generated names. It's useful when you need a quick directory without thinking about names.

## Features

- Create directories with random alphanumeric names
- Optional naming themes: scientists/engineers or pet names
- Create nested directories with controllable depth
- Use system temp directory or current working directory

## Usage
Basic: `qdir`

With options: qdir [options]

## Options
- `-d, --depth <depth>`    Set depth for nested directories
- `-l, --length <length>`  Set length for random string
- `-n, --name`             Use names instead of random string
- `-p, --pet`              Use pets instead of random string
- `-t, --tmp`              Use the system's temporary directory
- `-h, --help`             Print help
- `-V, --version`          Print version
