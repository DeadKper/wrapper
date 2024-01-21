#!/bin/sh
cargo search --limit 20 $@ \
	| grep -E -- "$(printf '%s|' $@ | sed -r 's/\|$//g')" \
	| grep -Ev '^... and' \
	| sed -r 's/^([a-z0-9_-]+) = "[0-9.]+"\s*# (.+)/\1\t\2/g'
