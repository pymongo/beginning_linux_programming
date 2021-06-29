#!/bin/bash

# set process's args/parameters
set arg1 arg2

for arg in $*; do
    if [[ $arg == *"arg"* ]]; then # if $arg contains "arg"
        echo $arg
    fi
done

# both $@ and $* are args exclude arg 0
# "$@"=args.join(" ")
# "$*"=args.join($IFS)
IFS=','
echo $@ # arg1 arg2
echo "$*" # "arg1,arg2" because $* use separator $IFS

exit 0
