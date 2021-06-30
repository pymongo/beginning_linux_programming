#!/bin/bash
echo "Please input your password: "
read -s user_input # read -s hide user input
echo $user_input
echo \$user_input # $user_input
echo '$user_input' # $user_input, 跟ruby一样单引号不替换变量
echo "$user_input"
exit 0
