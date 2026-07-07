#!/bin/bash

esta_logueado(){
    if [ "$(who | grep $1 | wc -l)" -gt 0 ]
    then
        echo true
    else
        echo false
    fi
}

cant_archivos(){
    if [ $(esta_logueado $1) == true ]
    then
        exit 58
    else
        home=$(cat /etc/passwd | grep $1 | cut -d: -f6)
        cont=0
        for e in ls $home
        do
            ruta="$home/$e"
            if [ -f ruta ]
            then
                ((cont++))
            fi
        done
        echo $cont
    fi
}

cant_procesos(){
    if [ $(esta_logueado $1) == false]
    then
        echo $(ps -U $1 | grep $2 | wc -l)
    fi
}

nombre=$1
filtro=$2
if [ $(esta_logueado $nombre) == true ]
then
    echo "$nombre: conectado"
    cant_procesos $nombre $filtro
else
    echo "$nombre: no conectado"
    cant_archivos $nombre
fi