#!/bin/bash
TSRCDIR=../csl-next/

cd ${TSRCDIR} || exit

# make sure to install quicktype

echo "Generating rust files ..."
echo ""
deno task npm
quicktype -s typescript -l rs --density dense --visibility public npm/src/style/options.ts -t OptionGroup -o ../csln-rs/generated/style/options.rs
quicktype -s typescript -l rs --density dense --visibility public npm/src/style/template.ts -t OptionGroup -o ../csln-rs/generated/style/template.rs
quicktype -s typescript -l rs --density dense --visibility public npm/src/reference.ts -t InputReference -o ../csln-rs/generated/bibliography/reference.rs

cd ../csln-rs || exit

echo ""
echo "Building rust binary ..."
echo ""
cargo build

echo ""
echo "Running rust binary ..."
echo ""
target/debug/csln-rs ${TSRCDIR}/examples/style.csl.yaml ${TSRCDIR}/examples/bibliography.yaml
