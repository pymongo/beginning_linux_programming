#!/bin/bash

# test function frequently used conditions:
# int_expr -eq int_expr
# -n string # string is not null and not empty
# -d file # file is a dir
# -f file # file is a regular file
if test -f ch02_args.sh
then
    echo "file is a regular file"
else
    echo "file not a regular file"
fi

# `test -f ch02_args.sh` is same as `[ -f ch02_args.sh ]`
if [ -d ch02_args.sh ]; then # need to add whitespace around `[` and `]`
    echo "file is a directory"
else
    echo "file not a directory"
fi

exit 0
