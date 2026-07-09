#!/bin/bash
pila=()

push(){
    pila=($1 "${pila[@]}")
}

pop(){
    aux=()
    elemento="${pila[0]}"
    for (( i=1; i<${#pila[@]}; i++ ))
    do
        aux=("${aux[@]}" "${pila[$i]}")
    done
    pila=("${aux[@]}")
    echo $elemento
}

length(){
    echo ${#pila[@]}
}

print(){
    for e in ${pila[@]}
    do
        echo $e
    done
}
a="a"
b="b"
push $a
push $b
push $a
push $a
push $a
push $b
push $a
push $a
push $a
push $b

echo "Elementos sacados: "
pop
pop
pop
echo "Mide: "
length

echo "--pila--"
print