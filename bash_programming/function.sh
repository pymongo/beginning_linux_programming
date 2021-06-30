#/bin/bash

fn_arg1() {
    local ret = "arg1=$1"
    return ret # or echo $ret
}

echo fn_arg1 arg1_is_1

exit 0
