#!/bin/bash

num=(10 3 5 7 9 3 5 4)

productoria(){
    total=${num[0]}
    pasos=${#num[@]}

    for (( i=1; i<pasos; i++ ))
    do
        (( total = total * ${num[$i]} ))
    done

    echo $total
}

productoria
