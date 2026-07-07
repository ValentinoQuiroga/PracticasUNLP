#!/bin/bash

if [ $# == 0 ]
then
    exit 58
fi

lista=($(ls /var/log | grep $1))
cont=0
for e in ${lista[@]}
do
    ruta="/var/log/$e"
    if [ -f "$ruta" ]
    then
        ((cont++))
    fi
done
echo $cont
exit 0