#!/bin/bash

# 在程序异常退出时执行些资源回收等操作
trap "rm -f ~/temp/$$.sock; exit 0" INT
touch ~/temp/$$.sock

# until true to break loop
until false; do
    echo $(date)
    sleep 1
done

exit 0

