const
    valor_alto = 9999;
    M = 4;
type
    Alumno = record
        nmbr:String[40];apll[40]:String;dni:integer;lgjo:integer;end;
    
    Nodo = record
        cant_claves: integer;
        claves: array[1..M] of integer;
        enlaces: array[1..M-1] of integer;
        hijos: array[1..M] of integer;
    end;

    arbolB = file of Nodo;
    archivo = file of Alumno;

var
    archivo: arcbolB;
begin
    writeln('Ejercicio de calculos y teoria');
end.