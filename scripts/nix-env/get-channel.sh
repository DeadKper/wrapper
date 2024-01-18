#!/bin/sh
channel=$(nix-channel --list | sed -r 's,\s.*,,g' | sort -r | head -n 1)
if test -z "$channel"
then
	echo nixpkgs
else
	echo $channel
fi
