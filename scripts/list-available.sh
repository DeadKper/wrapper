#!/bin/sh
DIR=$@
if test -z "$DIR"
then
	DIR="$(dirname $(readlink -f $0))"
fi

find $DIR -type f -name 'install.sh' -exec sh -c 'type $(basename $(dirname $0)) &> /dev/null && dirname $0' {} \; \
	| sed -r "s,^$DIR/,,g" \
	| sort -r
