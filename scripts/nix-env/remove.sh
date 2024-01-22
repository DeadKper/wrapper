#!/bin/sh
channel=$($(dirname $(readlink -f $0))/get-channel.sh $@)
if test -z "$channel"
then
	nix-env -e -- $@
else
	nix-env -f "<$channel>" -e -- $@
fi
