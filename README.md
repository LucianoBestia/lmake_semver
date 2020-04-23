# lmake_semver  

[comment]: # (lmake_semver cargo.toml data start)
version: 0.5.1  date: 2020-04-23 authors: Luciano Bestia  
**Includes cargo.toml data into md and then the content of md files into src/*.rs files as doc comments for later documentation generation.**

[comment]: # (lmake_semver cargo.toml data end)

First we want some data from cargo.toml to be included in the readme.md. To avoid data that is out of sync like version, authors and description.  
The we want to include the readme.md into the doc comments of the lib.rs or main.rs file.  
From this doc comments with `cargo doc` is then generated the documentation.  
We want to be all in sync with cargo.toml and readme.md.  
But also flexible to exclude what we don't need and to choose exactly where to include.  
The include can be executed many times, it will just replace the old content.  
The `lmake_semver` binary must be executed in the project root folder where is the cargo.toml file.  
It works only for single projects and not for workspaces.  

## install

Install from crates.io:  
`cargo install lmake_semver`  
Then you can use it in every rust project folder.  
No arguments needed to execute the util.  
Try manually with --help to see if it works in the current folder:  
`lmake_semver --help`  

FInd the documentation here:  
<https://lucianobestia.github.io/lmake_semver/>

## cargo.toml data

In the md file write these markers:  

```markdown
1 [comment]: # (lmake_semver cargo.toml data start)
2 [comment]: # (lmake_semver cargo.toml data end)
```

lmake_semver deletes the old lines between the markers  
and includes the date and the cargo.toml data:  
version, authors, description.  

## copy md content to doc comments

If you don't need all the content of the md file, you can exclude the lines
adding this markers:  

```markdown
1 [comment]: # (lmake_semver exclude start A)  
2 [comment]: # (lmake_semver exclude end A)  
```

In the rs file write these markers:  

```rust
1 // region: lmake_semver include "filename.md" //! A  
2 // endregion: lmake_semver include "filename.md" //! A  
```

lmake_semver deletes the old lines between the markers.  
Reads the filename of the md file to include. Exclude the eventual lines.  

Before each line adds the doc comment symbol as is defined in the marker.  
includes the new lines between the markers in the rs file.  

## Makefile.toml for cargo make  

I use `cargo make` to script the repetitive commands sequences.  
In `Makefile.toml` add a task like this:  

```toml
[tasks.doc]
description = "create docs from comments"
clear = true
dependencies = [
    "include-readme",
    "cargo-doc",
    "doc-copy",
]

[tasks.include-readme]
clear = true
private = true
description = "copy the content of readme.md into *.rs comments (for the docs)"
script= ["lmake_semver"]

[tasks.cargo-doc]
clear = true
private = true
description = "call cargo doc"
command="cargo"
args=["doc","--no-deps","--document-private-items"]

[tasks.doc-copy]
clear = true
private = true
description = "copy doc folder as docs (out of folder target), so it can be git committed"
script = [
    "\\rsync -avz --delete-after target/doc/*  docs/"
]
```

[comment]: # (lmake_semver exclude start A)  

Read the example.md to see a more complex scenario with more ms files and exclusion of lines.  

## TODO

Use the marker_name to match between different markers.  

[comment]: # (lmake_semver exclude end A)  
