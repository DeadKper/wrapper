#!/bin/sh
if test -z "$(echo $@ | grep -E -- '-f <\S+>')" -a -z "$(echo $@ | grep -E -- '\S+\.\S+')"
then
	channel=$(nix-channel --list | sed -r 's,\s.*,,g' | sort -r | head -n 1)
	if test -z "$channel"
	then
		echo nixpkgs
	else
		echo $channel
	fi
fi
