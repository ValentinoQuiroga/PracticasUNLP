arreglo=($(cat /etc/group | grep wheel | cut -d: -f4- | tr ':' ' '))

if [ $1 = "-b" ]
then
    if [ $(( ${#arreglo[@]} - 1 )) -ge $2 ]
    then
        echo ${arreglo[$2]}
    else
        echo "No hay elemento en esa posicion"
    fi
fi

if [ $1 = "-l" ]
then
    echo ${#arreglo[@]}
fi

if [ $1 = "-i" ]
then
    for u in "${arreglo[@]}"
    do
        echo $u
    done
fi