# SQL Database Mermaid Visualizer

A very simple tool written in Rust that takes PostgreSQL database files 
and generates a [Mermaid](https://mermaid-js.github.io/mermaid) entity relationship diagram.

## Installation

This project is published to [crates.io](https://crates.io/crates/sql_mermaid_visualizer) and thus can be installed using 
```bash
$ cargo install sql_mermaid_visualizer 
```
If you do not have cargo, the installation steps can be found 
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).
Additionally, make sure `~/.cargo/bin/` is in your PATH.

## Usage

```bash
USAGE:
  sqlvis [OPTIONS] --file <FILE>

OPTIONS:
  -f, --file <FILE>
  -h, --help                         Print help information
  -o, --output-file <OUTPUT_FILE>
  -V, --version                      Print version information
```

## Example

To print the Mermaid representation of the [./examples/simple.sql](examples/simple.sql) example 
to `stdout`:
```bash
$ sqlvis -f ./examples/simple.sql
```

Sample output of the above command: 
```md
erDiagram
  Student {
    INT StudentId
    INT ParentId
    VARCHAR30 Name
    INT Age
    VARCHAR25 Address
    VARCHAR20 Phone
  }
  
  Parent {
    INT ParentId
    INT StudentId
    INT PartnerId
    VARCHAR30 Name
    VARCHAR25 Address
    VARCHAR20 Phone
  }
  
  Student ||--|{ Parent : "FK_StudentParentId"
  Parent ||--|{ Student : "FK_ParentStudentId"
  Parent ||--|{ Parent : "FK_ParentPartnerId"
```

To output the Mermaid representation of the [./examples/big.sql](examples/big.sql) example
into `./big.md`:
```bash
$ sqlvis -f ./examples/big.sql -o ./big.md
```
