#!/bin/sh
nix --extra-experimental-features 'nix-command flakes' search nixpkgs nix-env
