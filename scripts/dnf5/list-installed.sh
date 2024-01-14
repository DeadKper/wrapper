#!/bin/bash
dnf5 list --installed | sed -r 's,\.\S+\s.*,,g'
