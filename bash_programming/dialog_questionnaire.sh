#!/bin/bash
set -u # print error when evaluate undefined variable

DIALOG_OK=0
DIALOG_CANCEL_OR_NO_OR_CTRL_C=1
DIALOG_ESC=255
DIALOG_STDERR_FILE="$HOME/temp/temp_dialog_stderr_output"

ui="${1:-cli_ui}"

# 因为alias不会转发所有参数，所以这里用了"$@"，或者用`alias my_dialog="/usr/bin/gdialog"``，但是alias不生效，所以还是用一个函数委托模式靠谱
my_dialog() {
    if [ "$ui" = "cli_ui" ]; then
        dialog "$@"
    else
        gdialog "$@"
    fi
}

my_dialog --title "Confirm" --yesno "Start a survey?" 0 0
case $? in
    $DIALOG_OK ) ;;
    $DIALOG_CANCEL_OR_NO_OR_CTRL_C )
        dialog --infobox "you select no" 0 0
        sleep 1
        dialog --clear
        exit 0
    ;;
esac

# 子shell中运行dialog看不到窗口，而且退出exit_code=DIALOG_ESC=255
my_dialog --inputbox "Input your name" 0 0 2> $DIALOG_STDERR_FILE
case $? in
    $DIALOG_OK ) input_name=$(cat $DIALOG_STDERR_FILE) ;;
    $DIALOG_CANCEL_OR_NO_OR_CTRL_C ) exit 0 ;;
esac

# bash hashmap(associative array), 前面不能加上`declare -A`否则会变成倒序
countries=(["1"]="China" ["2"]="Japan" ["3"]="Korean")
print_countries_kv_list() {
    # notice the ! to expand the keys
    for key in "${!countries[@]}"; do
        echo -n "$key ${countries[$key]} "
    done
}

my_dialog --title "Choose your country" --menu "$input_name, Choose your country" 0 0 0 $(print_countries_kv_list) 2> $DIALOG_STDERR_FILE
case $? in
    $DIALOG_OK ) country_select=$(cat $DIALOG_STDERR_FILE) ;;
    $DIALOG_CANCEL_OR_NO_OR_CTRL_C ) exit 0 ;;
esac

sleep 1
my_dialog --clear
echo "Your name    : $input_name"
echo "Your country : ${countries[$country_select]}"

exit 0
