# A demo of CSL-next codegen using quicktype

This is a demo repo of how code generation from the typescript models might work. It consists of:

1. a script to generate the rust `bibliography.rs`, `style.rs`, and `citation.rs` module files
2. a tiny `main.rs` that creates a `csln-rs` binary

Here's all that binary will do, but it demonstrates the deserialization and serialization of JSON or YAML `Style` or `InputBibliography` input files.

```console
❯ time target/debug/csln-rs style.csl.yaml bibliography.yaml
The name of the style is: "APA"
The number of entries in the bibliography is: 5

________________________________________________________
Executed in    2.74 millis    fish           external
   usr time    1.16 millis  406.00 micros    0.75 millis
   sys time    1.62 millis  115.00 micros    1.51 millis
```
