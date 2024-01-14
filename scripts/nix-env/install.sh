#!/bin/bash
nix-env -f '<nixpkgs>' -iA -- $@
