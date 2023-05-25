#!/bin/bash
TSRCDIR=../csl-next/

cd ${TSRCDIR} || exit

# make sure to install quicktype

echo "Generating rust files ..."
echo ""
deno task schemas
quicktype -s schema -l rs --density dense --visibility public schemas/csl-style-schema.json -t Style -o ../csln-rs/src/style.rs
quicktype -s schema -l rs --density dense --visibility public schemas/csl-inputbibliography-schema.json -t InputBibliography -o ../csln-rs/src/bibliography.rs
quicktype -s schema -l rs --density dense --visibility public schemas/csl-citation-schema.json -t Citation -o ../csln-rs/src/citation.rs

cd ../csln-rs || exit

echo ""
echo "Building rust binary ..."
echo ""
cargo build

echo ""
echo "Running rust binary ..."
echo ""
target/debug/csln-rs ${TSRCDIR}/examples/style.csl.yaml ${TSRCDIR}/examples/bibliography.yaml
