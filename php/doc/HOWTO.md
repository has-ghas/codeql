# Developer information

This document contains information about common development tasks.

## Building the tools from source

[Install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
(cd extractor && cargo build --release)
```

## Generating the database schema and QL library

The generated `ql/lib/php.dbscheme` and `ql/lib/codeql/php/ast/internal/TreeSitter.qll` files are included in the repository, but they can be re-generated as follows:

```bash
# First, build the release ^^
# Next, generate the dbscheme and TreeSitter.qll files
extractor/target/release/codeql-extractor-ruby generate --dbscheme ../ql/lib/php.dbscheme --library ../ql/lib/codeql/php/ast/internal/TreeSitter.qll
# Then, auto-format the generated QL library
codeql query format -i ql/lib/codeql/php/ast/internal/TreeSitter.qll
```
or
```
make dbscheme
```

## Building a CodeQL database for a PHP program

First, get an extractor pack. There are two options:

1. Either download the latest `codeql-php-pack` from Actions and unzip it twice, or
2. Run `scripts/create-extractor-pack.sh` (Linux/Mac) or `scripts\create-extractor-pack.ps1` (Windows PowerShell) and the pack will be created in the `extractor-pack` directory.

Then run

```bash
codeql database create <database-path> -l php -s <project-source-path> --search-path <extractor-pack-path>
```

## Running qltests

Run

```bash
codeql test run <test-path> --search-path <extractor-pack-path>
```

## Writing database upgrade scripts

See [this guide](prepare-db-upgrade.md).
