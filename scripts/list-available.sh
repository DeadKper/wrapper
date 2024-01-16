#!/bin/sh
find $@ -type f -name 'install.sh' -exec sh -c 'type $(basename $(dirname $0)) &> /dev/null && dirname $0' {} \; \
	| sed -r "s,^$@/,,g" \
	| sort -r
