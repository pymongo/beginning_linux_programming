#!/bin/bash
read user_input
echo $user_input
echo \$user_input # $user_input
echo '$user_input' # $user_input, 跟ruby一样单引号不替换变量
echo "$user_input"
exit 0
