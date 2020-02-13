# pinns.rs

### A simple utility to pin Linux namespaces

```
pinns v0.1.0
A simple utility to pin Linux namespaces

USAGE:
    pinns [FLAGS] [OPTIONS] --dir <DIRECTORY>

FLAGS:
    -h, --help       Prints help information
    -i, --ipc        Pin the IPC namespace
    -n, --net        Pin the network namespace
    -U, --user       Pin the user namespace
    -u, --uts        Pin the UTS namespace
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <DIRECTORY>      The directory for the pinned namespaces
    -l, --log-level <LEVEL>    The logging level of the application [default: info]  [possible values: trace, debug, info, warn, error, off]

More info at: https://github.com/saschagrunert/pinns.rs
```
