#!/bin/sh
nix-env -f '<nixpkgs>' -iA -- $@
