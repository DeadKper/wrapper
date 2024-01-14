#!/bin/bash
dnf5 search -- $@ | grep -E '^ ' | sed -r 's,^ (.*)\.\S+: (.*),\1\t\2,g'
