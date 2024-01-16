#!/bin/sh
find . -maxdepth 1 -mindepth 1 -type d -exec sh -c 'type "$(basename $0)" &> /dev/null && basename $0' {} \; | sort
