#!/bin/bash
nix-env -qa --installed '*' | sed -r 's,-[0-9.]+$,,g'
