# lucgrep

Project made following Rust Lang tutorial by the Rust Book.

This project is a CLI tool that search for a string inside a file.

## How to use it (Requires Cargo)
1 - Open a CLI (bash or GitBash)

2 - Initialize the program passing the executable path, string to look after and the path to the file.

``` bash
    cargo run [string] [file path]
    cargo run to ./poem.txt
```

2.1 - You can ignore upper case by setting the IGNORE_CASE flag to 1

``` bash
    cargo run [flag] [string] [file path]
    cargo run IGNORE_CASE=1 to ./poem.txt
```

3 - Results will be printed out on the terminal
