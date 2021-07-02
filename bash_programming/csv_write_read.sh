#!/bin/bash

write_user_id=1
write_username="foo"

csv_path="$HOME/temp/temp_data.csv"

echo $write_user_id,$write_username > $csv_path

IFS=","
read read_user_id read_username < $csv_path
IFS=" "

rm -f $csv_path

if [ $write_user_id = $read_user_id ] && [ $write_username = $read_username ]; then
    echo "csv write read success"
fi
exit 0
