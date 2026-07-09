#!/bin/bash
if [ $# -eq 0 ]
then
    exit 1
fi

lista="$@"
cant_impares=0
for e in ${lista[@]}
do
    resto=$(( $e % 2 ))
    if [ $resto -eq 0 ]
    then
        echo $e
    else
        (( cant_impares++ ))
    fi
done

echo $cant_impares