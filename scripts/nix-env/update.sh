#!/bin/bash
nix-env -f '<nixpkgs>' -uA -- $@
