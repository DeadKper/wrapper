#!/bin/sh
flatpak list --columns=name:f,application:f | tail -n +1
