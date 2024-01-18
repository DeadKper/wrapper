#!/bin/sh
nix-env -f "<$(./get-channel.sh)>" -e -- $@
