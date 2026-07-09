#!/bin/bash

usuarios=($(cat /etc/passwd | cut -d: -f1))
directorios=($(cat /etc/passwd | cut -d: -f6))

for (( i=0; i<${#usuarios[@]}; i++ ))
do
    u="${usuarios[$i]}"
    d="${directorios[$i]}"
    grupos=$(cat /etc/group | grep $u | wc -l)
    if [ -e $d ]
    then
        cant=$(find $d 2>/dev/null | wc -l)
    else
        d="XXX"
        cant=-1
    fi
    logs=($(ls /var/log))
    cant_logs=0
    for l in ${logs[@]}
    do
        ruta_log="/var/log/$l"
        if [ -f $ruta_log ]
        then
            cant_actual=$(cat $ruta_log 2>/dev/null| grep $u | wc -l)
            cant_logs=$((cant_logs + cant_actual))
        fi
    done
    echo "$u;$grupos;$d;$cant;$cant_logs"
done