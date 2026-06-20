#!/bin/bash

if [ $# -eq 0 ]; then
	exit 1
else
	echo "Hola probando"
	nombres=("$@")
	homes=$(cat /etc/passwd | cut -d: -f6)
	for ((i=0; i < $#; i++))
	do
		home_actual=$(echo "$homes" | grep "${nombres[i]}")
		if [ -d "$home_actual" ] && [ -d "$home_actual" ];
		then
			echo ${nombres[i]}
			echo "$home_actual"
			echo $(find $home_actual -name "*.docx" | wc -l )
		else
			echo ${nombres[i]}
			echo "Sin HOME"
			echo "0"
		fi
	done
fi
exit 0
