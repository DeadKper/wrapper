#!/bin/sh
flatpak search --columns=name:f,description:f,application:f -- $@ | tail -n +1
