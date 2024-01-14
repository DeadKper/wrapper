#!/bin/bash
nix-env -f '<nixpkgs>' -e -- $@
