# lmake_semver  

[comment]: # (lmake_semver cargo.toml data start)
version: 0.5.1  date: 2020-04-23 authors: Luciano Bestia  
**Includes cargo.toml data into md and then the content of md files into src/*.rs files as doc comments for later documentation generation.**

[comment]: # (lmake_semver cargo.toml data end)

Increments the patch or the minor in semver in cargo.toml.  

## Run

Run it with this arguments:  

`lmake_semver --increment=patch`  
`lmake_semver --increment=minor`  

## Install

`cargo install lmake_semver`  