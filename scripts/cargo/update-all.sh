#!/bin/sh
if type rustup &> /dev/null
then
	rustup update
fi
cargo install $(exec "$(dirname "$(readlink -f $0)")/list-installed.sh")
