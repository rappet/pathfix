[![Build Status](https://travis-ci.org/rappet/pathfix.svg?branch=master)](https://travis-ci.org/rappet/pathfix)

# PathFIX
Fixes the PATH variable mess.

Pathfix is a tool to generate the `$PATH` variable from configuration files.
In most cases you don't even need do edit the configuration,
because it comes with an included set of paths which will
be checked for existence and added on demand.

## Documentation

See `pathfix(1)` and `pathfix.toml(5)`

## Install

### From Binary

Install the binary from the release to `/usr/local/bin` or any other path and
execute Pathfix from and `.bashrc`, `.zshrc`, `.profile` or a similar file:

```shell script
# .bashrc/.zshrc/.profile/...
export PATH=$(/usr/local/bin/pathfix -D)
```

### With `dpkg`

Download the `.deb` package and install it:

```shell script
# dpkg -i pathfix_${VERSION}_amd64.deb
```

and execute `pathfix` as described above:

```shell script
# .bashrc/.zshrc/.profile/...
export PATH=$(/usr/bin/pathfix -D)
```

### Install with `cargo install`

```shell script
$ cargo install pathfix
```

### Build from source

clone the repo, compile it and install it:

```bash
cargo build --release
sudo cp target/release/pathfix /usr/local/bin/
```

## Configuration

You can edit the default configuration at `/etc/pathfix.toml`
or add your own paths to `~/.pathfix.toml`:

```toml
# ~/.pathfix.toml
# Paths to check if they are present and to add
paths = [
  "~/mybin"
]

# Overwrite or add environment variables which will be used
# when the $PATH will be build.
[env]
GOROOT = "/usr/local/go"
GOPATH = "/home/rappet/prog/go"
```

## CLI arguments

```
USAGE:
    pathfix [FLAGS] <--from-env|--included|--defaults>

FLAGS:
    -d, --dedup       Deduplicates the path
    -D, --defaults    Use recommended flags -des. Either -D, -e or -s must be set
    -e, --from-env    Includes path's from $PATH in environment
    -h, --help        Prints help information
    -i, --included    Searches included path's using inbuild configuration
    -l, --lines       Outputs line by line instead of the default colon seperated list
    -V, --version     Prints version information
```
