#!/bin/bash

# Inicializamos el array global vacío
lista=()

# Inicializa la lista con los argumentos que le pases a la función
init() {
        lista=("$@") 
}

# Recorre e imprime cada elemento de la lista
push() {
        # Usamos "${lista[@]}" para expandir TODOS los elementos del array
        for nodo in "${lista[@]}"
        do
                echo "$nodo" # Te faltaba el $
        done
}

# Muestra el último elemento y lo elimina del array (Función Pop/Last)
last() {
        # Validamos si la lista está vacía antes de sacar elementos
        if [ ${#lista[@]} -eq 0 ]; then
                echo "La lista está vacía"
                return
        fi

        # El último índice real es el tamaño total menos 1
        local ultimo_indice=$((${#lista[@]} - 1))
        
        # Guardamos el valor
        local ultimo_elemento=${lista[$ultimo_indice]}
        
        echo "Elemento sacado: $ultimo_elemento"
        
        # Eliminamos el último elemento del array
        unset "lista[$ultimo_indice]"
}

# --- PRUEBAS DEL SCRIPT ---

# Creamos variables comunes para probar (no uses $1 ni $2 aquí)
var1="a"
var2="b"

echo "=== Inicializando lista ==="
# Le pasamos las variables como argumentos a init
init "$var1" "$var2" "c"

echo "=== Contenido de la lista ==="
# Llamamos a las funciones sin paréntesis
push 

echo "=== Sacando el último elemento ==="
last

echo "=== Lista después de modificarla ==="
push
