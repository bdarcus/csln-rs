# A demo of CSL-next codegen using quicktype

This is a demo repo of how code generation from the typescript models might work. It consists of:

1. a script to generate the rust `reference.rs`, `style.rs`, and `citation.rs` module files
2. a tiny `main.rs` that creates a `csln-rs` binary

Here's all that binary will do, but it demonstrates the deserialization and serialization.

```console
❯ target/debug/csln-rs src/style.csl.json src/bibliography.json
The name of the style is: "APA"
The number of entries in the bibliography is: 5
```
