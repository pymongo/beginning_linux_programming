#!/bin/bash

echo -n "Please input yes or no: "
read input

case "$input" in
    yes | Yes | YES | y | Y ) echo "yes";;
    # no or n or N
    no | [nN] ) echo "no";;
    * )
        echo "default: break"
        exit 1
        ;;
esac

exit 0

