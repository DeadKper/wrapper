#!/bin/sh
channel=$(./get-channel.sh $@)
if test -z "$channel"
then
	nix-env -uA -- $@
else
	nix-env -f "<$channel>" -uA -- $@
fi
