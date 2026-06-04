const
    valor_alto = 9999;
type
    Libro = record
        cdgo:integer;gnro:String;ttlo:String;autr:String;pags:integer;prec:double;end;
    
    Libreria = file of Libro;

procedure menu(var opcion: integer);
begin
    writeln('.................................');
    writeln('1. Crear y cargar archivo');
    writeln('2. Ver opciones de mantenimiento');
    writeln('3. Exportar datos a texto');
    writeln('0. Cerrar programa');
    writeln('.................................');

    writeln('Ingrese su opcion: ');
    readln(opcion);
end;

procedure recibir_libro(var l: Libro);
begin
    writeln('Codigo del libro (-1 para salir): ');
    readln(l.cdgo);
    if (l.cdgo <> -1) then begin
        writeln('Genero: ');
        readln(l.gnro);
        writeln('Titulo: ');
        readln(l.ttlo);
        writeln('Autor: ');
        readln(l.autr);
        writeln('Cantidad de paginas: ');
        readln(l.pags);
        writeln('Precio: ');
        readln(l.prec);
    end;
end;

procedure crear_cargar_archivo(var lbr: Libreria);
var
    dato, cabecera: Libro;
begin
    rewrite(lbr);
    
    cabecera.cdgo:= 0;
    write(lbr, cabecera);

    recibir_libro(dato);
    while(dato.cdgo <> -1) do begin
        write(lbr, dato);
        recibir_libro(dato);
    end;
end;

procedure opciones_de_mantenimiento(var lbr: Libreria);
begin
    
end;
var
    opcion: integer;
    lbr: Libreria;
    nlbr: String;
begin
    nlbr:= 'libreria';
    assign(lbr,nlbr);
 
    opcion := -1;
    while (opcion <> 0) do begin
        menu(opcion);

        case (opcion) of 
            1: crear_cargar_archivo(lbr);
            //2: opciones_de_mantenimiento(lbr);
            //3: exportar_a_texto(lbr);
        end;

    end;
end.