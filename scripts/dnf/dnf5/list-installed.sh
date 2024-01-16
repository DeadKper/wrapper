#!/bin/sh
dnf5 list --installed | sed -r 's,\.\S+\s.*,,g'
