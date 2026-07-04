const
    valor_alto = 9999;
type
    Libro = record
        cdgo:integer;gnro:String;ttlo:String;autr:String;pags:integer;prec:double;end;
    
    Libreria = file of Libro;

procedure leer_l(var lbr: Libreria; var dato: Libro);
begin
    if (not EOF(lbr)) then read(lbr, dato)
    else dato.cdgo := valor_alto;
end;

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

procedure menu_mantenimiento(var opcion: integer);
begin
    writeln('.................................');
    writeln('1. Dar de alta un libro');
    writeln('2. Modificar un libro');
    writeln('3. Dar de baja un libro');
    writeln('0. Volver al menu principal');
    writeln('.................................');

    writeln('Ingrese su opcion: ');
    readln(opcion);
end;

procedure recibir_modificaciones(var l: libro);
var
  entrada: string;
  codigo: integer;
  aux_int: integer;
  aux_double: double;
begin
  { --- GÉNERO (String) --- }
  writeln('Genero (Actual: ', l.gnro, '): ');
  readln(entrada);
  if entrada <> '' then
    l.gnro := entrada; { Si ingresó algo, se modifica. Si no, queda igual }

  { --- TÍTULO (String) --- }
  writeln('Titulo (Actual: ', l.ttlo, '): ');
  readln(entrada);
  if entrada <> '' then
    l.ttlo := entrada;

  { --- AUTOR (String) --- }
  writeln('Autor (Actual: ', l.autr, '): ');
  readln(entrada);
  if entrada <> '' then
    l.autr := entrada;

  { --- CANTIDAD DE PÁGINAS (Integer) --- }
  writeln('Cantidad de paginas (Actual: ', l.pags, '): ');
  readln(entrada);
  if entrada <> '' then
  begin
    Val(entrada, aux_int, codigo);
    if codigo = 0 then
      l.pags := aux_int
    else
      writeln('Valor invalido. Se conserva el valor anterior.');
  end;

  { --- PRECIO (Double) --- }
  writeln('Precio (Actual: ', l.prec:0:2, '): ');
  readln(entrada);
  if entrada <> '' then
  begin
    Val(entrada, aux_double, codigo);
    if codigo = 0 then
      l.prec := aux_double
    else
      writeln('Valor invalido. Se conserva el valor anterior.');
  end;
end;

procedure recibir_libro(var l: Libro);
var
    cdgo: integer;
begin
    writeln('Codigo del libro (negativo para salir): ');
    readln(cdgo);
    if (cdgo > 0) then begin
        l.cdgo:= cdgo;
        l.gnro:= '--';
        l.ttlo:= '--';
        l.autr:= '--';
        l.pags:= 0;
        l.prec:= 0;
        {writeln('Genero: ');
        readln(l.gnro);
        writeln('Titulo: ');
        readln(l.ttlo);
        writeln('Autor: ');
        readln(l.autr);
        writeln('Cantidad de paginas: ');
        readln(l.pags);
        writeln('Precio: ');
        readln(l.prec);}
    end;
end;

procedure generar_cabecera(var c: Libro);
begin
    c.cdgo:= 0;
    c.gnro:= '--';
    c.ttlo:= '--';
    c.autr:= '--';
    c.pags:= 0;
    c.prec:= 0;
end;

procedure crear_cargar_archivo(var lbr: Libreria);
var
    dato, cabecera: Libro;
begin
    rewrite(lbr);
    
    generar_cabecera(cabecera);
    write(lbr, cabecera);

    recibir_libro(dato);
    while(dato.cdgo <> -1) do begin
        write(lbr, dato);
        recibir_libro(dato);
    end;
end;

procedure alta_libro(var lbr: Libreria);
var
    dato, libro_nuevo, puntero: Libro;
    pos, pos_sig: integer;
begin
    reset(lbr);
    recibir_libro(libro_nuevo);
    leer_l(lbr, dato);

    if (dato.cdgo = 0) then begin
        seek(lbr, fileSize(lbr));
        write(lbr, libro_nuevo);
    end
    else begin
        while(dato.cdgo < 0) do begin
            pos:= filePos(lbr) - 1;
            seek(lbr, dato.cdgo * -1);
            leer_l(lbr, dato);
        end;
        seek(lbr, filePos(lbr) - 1);
        write(lbr, libro_nuevo);
        generar_cabecera(puntero);
        seek(lbr, pos);    
        write(lbr, puntero);
    end;
    close(lbr);
end;

procedure mod_libro(var lbr: Libreria);
var
    codigo: integer;
    dato: Libro;
begin
    writeln('Ingrese codigo del libro a modificar');
    readln(codigo);
    reset(lbr);
    leer_l(lbr, dato);
    while(dato.cdgo <> valor_alto) and (dato.cdgo <> codigo) do begin
        leer_l(lbr, dato);
    end;
    if (dato.cdgo = codigo) then begin
        recibir_modificaciones(dato);
        seek(lbr, filePos(lbr) - 1);
        write(lbr, dato);
    end
    else writeln('Libro no encontrado');
    close(lbr);
end;

procedure baja_libro(var lbr: Libreria);
var
    dato: Libro;
    codigo, pos: integer;
begin
    writeln('Ingrese codigo del libro a eliminar');
    readln(codigo);
    reset(lbr);

    leer_l(lbr, dato);
    while(dato.cdgo <> valor_alto) and (dato.cdgo <> codigo) do leer_l(lbr, dato);

    if (dato.cdgo = codigo) then begin
        pos:= filePos(lbr) - 1;
        dato.cdgo:= 0;
        seek(lbr, pos);
        write(lbr, dato);

        reset(lbr);
        leer_l(lbr, dato);
        while(dato.cdgo < 0) do begin
            seek(lbr, dato.cdgo * (-1));
            leer_l(lbr, dato);
        end;
        dato.cdgo:= pos * -1;
        seek(lbr, filePos(lbr) - 1);
        write(lbr, dato);
    end
    else writeln('Libro no encontrado');
    close(lbr);
end;

procedure opciones_de_mantenimiento(var lbr: Libreria);
var
    opcion: integer;
begin

    menu_mantenimiento(opcion);

    while(opcion <> 0) do begin
        case (opcion) of
            1: alta_libro(lbr);
            2: mod_libro(lbr);
            3: baja_libro(lbr);
        end;
        menu_mantenimiento(opcion);
    end;
end;

procedure exportar_a_texto(var lbr: Libreria);
var
    dato: Libro;
    texto: Text;
    ntexto: String;
begin
    ntexto:= 'libros.txt';
    assign(texto, ntexto);
    rewrite(texto);

    reset(lbr);

    leer_l(lbr, dato);
    while(dato.cdgo <> valor_alto) do begin
        if (dato.cdgo > 0) then begin
            writeln(texto, dato.cdgo,' ', dato.ttlo);
            writeln(texto, dato.pags,' ', dato.autr);
            writeln(texto, dato.prec:0:2,' ', dato.gnro);
        end;
        leer_l(lbr, dato);
    end;
    close(texto);
    close(lbr);
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
            2: opciones_de_mantenimiento(lbr);
            3: exportar_a_texto(lbr);
        end;

    end;
end.