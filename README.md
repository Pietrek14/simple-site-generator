# simple-site-generator

A simple HTML site generator written in Rust.

## Syntax

 - \* - a paragraph (`<p>`)
 - \+ - an unordered list element (`<li>`)

The title is inferred from the filename.

## Usage

 - From source code: `cargo run *filename*`
 - From compiled program: `ssg *filename*`