# Pretty copy

Pretty copy(shortened `pcp`) is utility for copying files which aims to provide better UI and more features compared to
standard UNIX `cp` utility.
<br>

![showcase](img/output.gif)

# Installing

## Cargo

You may use `cargo` to install `pretty-copy` crate, though in this case **binary would be named** `pretty-copy`, not
`pcp` so you would need to alter name manually:

```shell
cargo install pretty-copy
cd /path/to/your/cargo/bin
mv pretty-copy pcp
```

## Arch-based

On Arch distributions you may use AUR helper(e.g. `yay` to install `pretty-copy` package):

```shell
yay -S pretty-copy
```

or clone this repository and `makepkg`:

```shell
git clone https://github.com/0x50f13/pcp
cd pcp/pkg
makepkg -si
```

## Others

Currently on other distributions you may download x86_64 binary from
[releases](https://github.com/0x50f13/pcp/releases) or [build for yourself](#build)

# Build

For building you need Rust and Cargo. To build clone this repository and use cargo:

```shell
git clone https://github.com/0x50f13/pcp
cd pcp
cargo build --release
```

And then copy resulting file to any of directories from your [`$PATH`](https://en.wikipedia.org/wiki/PATH_(variable)):

```shell
# Here /usr/bin is just an example
cp target/release/pcp /usr/bin
```

# Usage
**Usage:** `pcp [OPTIONS] <SRCS>... <DEST>`

| Argument/Option                           | Description                                                                  |
|-------------------------------------------|------------------------------------------------------------------------------|
| `<SRCS>...`                               | Source file/directories                                                      |
| `<DEST>`                                  | Destination file/directory                                                   |
| `-r, --recursive`                         | Copy directories recursively                                                 |
| `--no-progress`                           | Do not show progress                                                         |
| `--fail-fast`                             | Fail on first error                                                          |
| `--max-chunks-number <MAX_CHUNKS_NUMBER>` | Maximum number of cached chunks of file stored in memory [**default:** 1024] |
| `-h, --help`                              | Print help                                                                   |

# Features
## Current
* [x] Copy files with progress
* [x] Copy directories

## Future
* [ ] Copying files via HTTP and HTTPS protocols
* [ ] Metadata copying(e.g. SELinux labels)
* [ ] Copying files/directories via SCP
* [ ] Copying files/directories via FTP and SFTP
* [ ] Reflinking files when supported
* [ ] Writing directly to devfs, so utility may burn file to drive straight away
* [ ] Calculate directory size before copying it
* [ ] Qt or GTK progress window(or Tcl, or maybe even direct rendering with GL?)
* [ ] Optional plugins support

# Development
## Documentation
### Online
**TODO**
### Generation
Use `cargo doc` for documentation generation.
## Short abstract
The program possess currently 3 dynamical components which have corresponding traits:
* `Reader` implements reading files(and their metadata)
* `Write` implements writing files
* `ProgressDisplay` implements showing progress to user
* Actually one more is planned: `InstantCopyHelper` which would help to determine whether file can be instantly reflink'd

The general algorithm is that we have two coroutines and channel between them. 
One coroutine reads file another one writes to it. The coroutine which writes additionally
updates progress which is done synchronously(so in future UI thread may be added via mpsc channel)
