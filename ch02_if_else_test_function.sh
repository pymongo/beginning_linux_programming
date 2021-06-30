#!/bin/bash

echo -n "Please input a filename: " # echo -n 参数能避免echo换行，或者用printf函数去避免换行
read filename

# test function frequently used conditions:
# `test -f ch02_args.sh` is same as `[ -f ch02_args.sh ]`
# int_expr -eq int_expr
# -a file # file exist
# -d file # file is a dir
if test -f "$filename"; then # 双引号包起来filename避免输入为NULL时if判断异常
    echo "regular file"
elif [ -d "$filename" ]; then # need to add whitespace around `[` and `]`
    echo "directory"
elif [ -a "$filename" ]; then
    echo "file exist"
else
    echo "file not exist"
fi

echo -e "\a" # -e means escape '\a' to alert(bell)

exit 0
