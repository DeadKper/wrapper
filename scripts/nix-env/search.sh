#!/bin/sh
nix --extra-experimental-features 'nix-command flakes' search nixpkgs -- $@ > ~/.cache/nix-env.search.tmp \
	&& cat ~/.cache/nix-env.search.tmp \
	| sed -r 's,^\*\s+([^.]+.){2},,g;s,\s\(\S+\)$,,g;s,^\s+,\t,g;s,\x1b\[[0-9;]*m,,g' \
	| sed -rz 's,\n\t,\t,g' \
	| grep -vE '^$'
