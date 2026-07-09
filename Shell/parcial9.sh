#!/bin/bash

vec1=(3 6 9 12 15 18 21 24 27)
vec2=(1 2 3 4 5 6 7 8 9)

n=${#vec1[@]}

for (( i=0; i<n; i++))
do
    suma=$(( ${vec1[$i]} + ${vec2[$i]}))
    echo "La suma de los elementos en la posicion $i de los vectores es $suma"
done