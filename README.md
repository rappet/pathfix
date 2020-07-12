# PathFIX
Fixes the PATH variable mess.

Patfix will

- remove duplicate entries in $PATH
- automatically find other entries for $PATH

## Build

clone the repo, compile it and install it:

```bash
cargo build --release
sudo cp target/release/pathfix /usr/local/bin/
```

## Usage
Append at the end of .bashrc, .profile or similar:

```bash
export PATH=$(/path/to/pathfix)
```
## Help

Output of `pathfix --help`

```
pathfix 1.0
Raphael Peters <raphael.r.peters@gmail.com>
Fixes the $PATH mess

USAGE:
    pathfix [FLAGS]

FLAGS:
    -d, --dedup             Deduplicates the path. Default set if -R is not set
    -h, --help              Prints help information
    -l, --lines             Outputs line by line instead of the default colon seperated list
    -R, --no-recommended    Disables recommended flags
    -V, --version           Prints version information
```
