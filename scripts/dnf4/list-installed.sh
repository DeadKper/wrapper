#!/bin/sh
dnf4 list --installed | sed -r 's,\.[^. ]+.*,,g'
