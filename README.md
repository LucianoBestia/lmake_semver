# lmake_semver  

[comment]: # (lmake_readme cargo.toml data start)
version: 0.1.6  date: 2020-04-24 authors: Luciano Bestia  
**Increments the patch or minor version in cargo.toml.**

[comment]: # (lmake_readme cargo.toml data end)

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
`clear; cargo make`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies, including this one.  
Please, spread this info.  
On the web use url to read crate reviews example:  
<bestia.dev/cargo_crev_web/query/num-traits>  

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
