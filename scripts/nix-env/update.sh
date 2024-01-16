#!/bin/sh
nix-env -f '<nixpkgs>' -uA -- $@
