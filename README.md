# libschnauzer

![](https://github.com/Arsynth/schnauzer/actions/workflows/rust.yml/badge.svg)

Schnauzer is both library and tool for parsing mach-o files

### Features

* Zero copy. Does not loads whole binary into memory. Uses iterators to list potentially large amount of items
* Endian aware
* Implements derive macro for automatic field enumeration, that, for example, very convenient for printing arbitary load commands. There even no need to write large `match` blocks for any type of load command
* Prints file structure in color for better user experience

### Installation

```shell
cargo install schnauzer
```

### Arguments

Since `version 0.2.4`, `schnauzer` requires `-p` or `--path` argument to specify path to file

### Supported commands
```shell
# Prints almost all binary info
schnauzer -p path_to_binary

# Prints symtab
schnauzer syms -p path_to_binary

# Prints relative paths
schnauzer rpaths -p path_to_binary

# Prints used dynamic libraries
schnauzer dylibs -p path_to_binary

# Prints all the segments with sections
schnauzer segs -p path_to_binary

# Prints the fat archs
schnauzer fat -p path_to_binary

# Prints headers
schnauzer headers -p path_to_binary

```

### Example output

Put something like in your console:

```shell
schnauzer -p /bin/cat
```

And you get:

![example output №1](https://github.com/Arsynth/schnauzer/blob/master/readme_res/example_output_1.png?raw=true)
![example output №2](https://github.com/Arsynth/schnauzer/blob/master/readme_res/example_output_2.png?raw=true)

Some info may be too big to be printed with other info that may be inconvenient. So there separate subcommand to print all `nlist`s:

```shell
schnauzer syms -p path_to_binary
```

![example syms output](https://github.com/Arsynth/schnauzer/blob/master/readme_res/example_output_syms_3.png?raw=true)

### Documentation
docs.rs/schnauzer/0.2.4

### Usage

```toml
[dependencies]
schnauzer = "0.2.4"
```

### Examples

Simple debug print

```rust
use schnauzer::ObjectType;
use schnauzer::Parser;
use std::path::Path;

fn main() {
    let mut args = std::env::args();
    let _exec_name = args.next();

    let path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Not enough arguments. Provide a valid path to binary");
            std::process::exit(1);
        }
    };
    let path = Path::new(&path);

    let parser = match Parser::build(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Could not create parser at '{:?}': {e}", path);
            std::process::exit(1);
        }
    };

    let object = match parser.parse() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error while parsing: {:#?}", e);
            std::process::exit(1);
        }
    };

    handle_object(object);
}

fn handle_object(obj: ObjectType) {
    println!("***Object***");
    println!("{:#?}", obj);
}
```

Using `AutoEnumFields` derive (code taken from `src/main.rs`)

```rust
let h = macho.header();
for field in h.all_fields() {
    out_dashed_field(field.name, field.value, level);
}
```

# Contacts

You may email me: 
[arsynthdev@gmail.com](mailto:arsynthdev@gmail.com)

[GitHub profile](https://github.com/Arsynth)