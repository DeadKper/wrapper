#!/bin/sh
nix-env -f '<nixpkgs>' -e -- $@
