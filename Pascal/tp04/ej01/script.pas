const
    valor_alto = 9999;
    M = 4;
type
    Alumno = record
        nmbr:String;apll:String;dni:integer;lgjo:integer;end;
    
    Nodo = record
        cant_datos: integer;
        datos: array[1..M-1] of Alumno;
        hijos: array[q..M] of Integer;
    end;

    arbolB = file of Nodo;

var
    archivo: arcbolB;
begin
    writeln('Ejercicio de calculos y teoria');
end.