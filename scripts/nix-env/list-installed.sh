#!/bin/sh
nix-env -qa --installed '*' | sed -r 's,-[0-9.]+$,,g'
