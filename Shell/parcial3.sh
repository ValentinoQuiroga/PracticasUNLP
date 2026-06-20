#!/bin/bash

if [ -f $1 ]
then
    echo "Existe"
    permisos="$(ls $1 -l | cut -c 1-4)"
    echo "Permisos $permisos"
    hay_permiso_x=$( echo $permisos | grep -o "x" | wc -l)
    if [ $hay_permiso_x -eq 1 ]
    then
        echo "Quitando permisos de ejecucion"
        chmod u-x $1
    fi

else
    echo "No existe"
fi