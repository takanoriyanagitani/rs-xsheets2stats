#!/bin/sh

wsm="./rs-xsheets2stats.wasm"

ifile="./sample.d/input.xlsx"
gname="/guest.d/read-write.d/input.xlsx"

geninput() {
	echo creating input file...
	export ifile="${ifile}"
	mkdir -p sample.d
	python3 geninput.py
}

test -f "${ifile}" || geninput

run_wasm() {
	wasmtime run \
		--dir="${PWD}/sample.d::/guest.d/read-write.d" \
		"${wsm}" \
		"$@"
}

echo "--- row count (default) ---"
run_wasm "${gname}"

echo "\n--- row count (explicit) ---"
run_wasm --count-rows "${gname}"

echo "\n--- cell count (all) ---"
run_wasm --count-cells-all "${gname}"

echo "\n--- cell count (non-empty) ---"
run_wasm --count-cells-non-empty "${gname}"

echo "\n--- byte count ---"
run_wasm --count-bytes "${gname}"
