const
    VA = 9999;
type
    Distribucion = record
        nom: String[20]; aaaa: integer; version: String[20]; devs: integer; desc: String[20];
    end;

    Maestro = file of Distribucion;

procedure leer(var mae: Maestro; var dato: Distribucion);
begin
    if (not EOF(mae)) then read(mae, dato)
    else dato.devs := VA;
end;

function buscarDistribucion(var mae: Maestro; nombre: String): integer;
var
    dato: Distribucion;
begin
    if (fileSize(mae) > 1) then begin
        seek(mae, 1);
        leer(mae, dato);
        while (dato.devs <> VA) and (dato.nom <> nombre) do leer(mae, dato);
        if (dato.nom = nombre) and (dato.devs > 0) then buscarDistribucion:= (filePos(mae) - 1)
        else buscarDistribucion:= -1;
    end
    else buscarDistribucion:= -1;
end;

procedure altaDistribucion(var mae: Maestro; distro: Distribucion);
var
    pos: integer;
    dato: Distribucion;
begin
    pos:= buscarDistribucion(mae, distro.nom);
    reset(mae);
    if (pos <> -1) then writeln('ya existe la distribucion')
    else begin
        leer(mae, dato);
        if (dato.devs = 0) then begin
            seek(mae, fileSize(mae));
            write(mae, distro);
        end
        else begin
            while (dato.devs <> VA) and (dato.devs < 0) do begin
                pos:= filePos(mae) - 1;
                seek(mae, (dato.devs * -1));
                leer(mae, dato);
            end;

            if (dato.devs = 0) then begin
                seek(mae, filePos(mae) - 1);
                write(mae, distro);

                seek(mae, pos);
                dato.devs:= 0;
                write(mae, dato);
            end;
        end;
    end;
end;

procedure bajaDistribucion(var mae: Maestro; nombre: String);
var
    pos: integer;
    dato: Distribucion;
begin
    pos:= buscarDistribucion(mae, nombre);
    reset(mae);
    if (pos = -1) then writeln('Distribucion no existente')
    else begin
        leer(mae, dato);
        while (dato.devs < 0) do begin
            seek(mae, dato.devs * -1);
            leer(mae, dato);
        end;

        seek(mae, filePos(mae) - 1);
        dato.devs := pos * -1;
        write(mae, dato);

        seek(mae, pos);
        dato.devs:= 0;
        write(mae, dato);
    end;
end;

procedure imprimirBusqueda(pos: integer);
begin
    if (pos = -1) then writeln('No se encontro')
    else writeln('Esta en ', pos);
end;

var
    mae: Maestro;
    dato: Distribucion;
    pos: integer;
Begin
    assign(mae, 'maestro');

{    rewrite(mae);
    dato.nom:= 'aaa';
    dato.aaaa:= 0;
    dato.version:= 'bbb';
    dato.devs:= 0;
    dato.desc:= 'ccc';

    write(mae, dato);
}
    reset(mae);

    dato.nom:= 'a';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'b';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'c';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'd';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    bajaDistribucion(mae, 'c');
    bajaDistribucion(mae, 'b');
    bajaDistribucion(mae, 'a');

    dato.nom:= 'm';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'o';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'p';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    dato.nom:= 'q';
    dato.aaaa:= 2000;
    dato.version:= '1.1';
    dato.devs:= 10;
    dato.desc:= 'aaaa';
    altaDistribucion(mae, dato);

    imprimirBusqueda(buscarDistribucion(mae, 'o'));
    imprimirBusqueda(buscarDistribucion(mae, 'p'));
    imprimirBusqueda(buscarDistribucion(mae, 'm'));
    imprimirBusqueda(buscarDistribucion(mae, 'd'));
    imprimirBusqueda(buscarDistribucion(mae, 'q'));


    close(mae);
end.
