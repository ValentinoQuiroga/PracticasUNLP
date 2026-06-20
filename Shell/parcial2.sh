#!/bin/bash

lista=()
elemento_devuelto=""

init() {
        lista=("$@") 
}

push() {
        lista=($1 "${lista[@]}")
}

# Muestra el último elemento y lo elimina del array (Función Pop/Last)
last() {
 # Validamos si la lista está vacía antes de sacar elementos
        if [ ${#lista[@]} -eq 0 ]; then
                echo "La lista está vacía"
                return
        fi

        local ultimo_indice=$((${#lista[@]} - 1))

        elemento_devuelto="${lista[$ultimo_indice]}"

        unset "lista[$ultimo_indice]"
}

first() {
 # Validamos si la lista está vacía antes de sacar elementos
        if [ ${#lista[@]} -eq 0 ]; then
                echo "La lista está vacía"
                return
        fi

        elemento_devuelto="${lista[0]}"

        unset "lista[0]"
}

size() {
        return ${#lista[@]}
}
       
print() {
        for nodo in ${lista[@]}
        do
                echo " - $nodo"
        done
}
# --- PRUEBAS DEL SCRIPT ---

var1="a"
var2="b"

init "$var1" "$var2" "c"

push "$var1"

print

last

echo "Se saco el ultimo: $elemento_devuelto"

print

first

echo "Se saco el primero: $elemento_devuelto"

print

size

echo "Tamaño: $?"
