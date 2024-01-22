#!/bin/sh
channel=$($(dirname $(readlink -f $0))/get-channel.sh $@)
if test -z "$channel"
then
	nix-env -iA -- $@
else
	nix-env -f "<$channel>" -iA -- $@
fi
