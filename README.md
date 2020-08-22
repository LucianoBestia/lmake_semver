# lmake_semver  

[comment]: # (lmake_cargo_toml_to_md start)

***version: 0.1.11  date: 2020-08-22 authors: Luciano Bestia***  
**Increments the patch or minor version in Cargo.toml.**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-184-green.svg)](https://github.com/LucianoBestia/lmake_semver/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-48-blue.svg)](https://github.com/LucianoBestia/lmake_semver/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-64-purple.svg)](https://github.com/LucianoBestia/lmake_semver/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_semver/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_semver/)

[comment]: # (lmake_lines_of_code end)

## Install

`cargo install lmake_semver`  

## Run

Run it with this arguments:  

`lmake_semver --increment=patch`  
`lmake_semver --increment=minor`  

## Development

Documentation:  
<https://lucianobestia.github.io/lmake_semver>  
List of prepared make tasks for development: build, run, doc, publish,...  
`cargo make`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies, including this one.  
Please, spread this info.  
On the web use url to read crate reviews example:  
<web.crev.dev/rust-reviews/crate/num-traits/>  

## Tasks in Makefile.toml

Libraries use semver. On every build release you can increment patch.  

```toml
[tasks.release]
description = "cargo build release"
clear = true
dependencies = [
    "semver_increment_patch",
    "build_release",
]

[tasks.semver_increment_patch]
clear = true
private = true
description = "increment semver patch"
script= ["lmake_semver --increment=patch"]
```
