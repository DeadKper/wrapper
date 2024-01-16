#!/bin/sh
dnf4 search $@ | grep -Ev '^=+'| sed -r 's,\.[^. ]+ : ,\t,g'
