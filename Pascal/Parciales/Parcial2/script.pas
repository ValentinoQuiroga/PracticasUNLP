const
    VA = 9999;
type
    Aplicacion = record
        cod:integer;nom:String[50];origen:String[50];
        precio:double; licDisp: integer; licMin: integer;
    end;

    Venta = record
        cod: integer; cant: integer;
    end;

    Maestro = file of Aplicacion;
    Detalle = file of Venta;

    ArrDet = array[1..20] of Detalle;
    ArrReg = array[1..20] of Venta;

procedure leer_m(var mae: Maestro; var dato: Aplicacion);
begin
    if (not EOF(mae)) then read(mae, dato)
    else dato.cod := VA;
end;

procedure leer_d(var det: Detalle; var dato: Venta);
begin
    if (not EOF(det)) then read(det, dato)
    else dato.cod := VA;
end;

procedure minimo(var archivos: ArrDet; var registros: ArrReg; var min: Venta);
var
    pos, i: integer;
begin
    pos:= -1;
    min.cod:= VA;
    for i:= 1 to 20 do begin
        if registros[i].cod < min.cod then begin
            min:= registros[i];
            pos:= i;
        end;
    end;
    if min.cod <> VA then leer_d(archivos[pos], registros[pos]);
end;

procedure inicializar_detalles(var archivos: ArrDet; var registros: ArrReg);
var
    i: integer;
begin
    for i:= 1 to 20 do leer_d(archivos[i], registros[i]);
end;

procedure procedimiento(var mae: Maestro; var archivos:ArrDet; var registros:ArrReg; var informe: Text);
var
    i: integer;
    datoDet, ventaActual: Venta;
    datoMae: Aplicacion;
    montoDiario: double;
begin
    reset(mae);
    rewrite(informe);
    for i:= 1 to 20 do reset(archivos[i]);
    inicializar_detalles(archivos, registros);
    
    datoDet.cod:= 9999;
    minimo(archivos, registros, datoDet);
    
    leer_m(mae, datoMae);
    while (datoDet.cod <> VA) do begin
        ventaActual.cod:= datoDet.cod;
        ventaActual.cant:= 0;
        while (datoDet.cod = ventaActual.cod) and (datoDet.cod <> VA) do begin
            ventaActual.cant:= ventaActual.cant + datoDet.cant;
            minimo(archivos,registros,datoDet);
        end;

        while(datoMae.cod <> ventaActual.cod) and (datoMae.cod <> VA) do begin
            leer_m(mae, datoMae);
        end;

        datoMae.licDisp:= datoMae.licDisp - ventaActual.cant;
        montoDiario:= datoMae.precio * ventaActual.cant;

        if ( montoDiario > 10000) then begin
            writeln(informe, datoMae.cod, ' ', datoMae.nom);
            writeln(informe, montoDiario:0:2, ' ', datoMae.origen);
        end;

        seek(mae, filePos(mae) - 1);
        write(mae, datoMae);
    end;
    close(mae);
    close(informe);
    for i:= 1 to 20 do close(archivos[i]);
end;

var
    mae: Maestro;
    archivos: ArrDet;
    registros: ArrReg;
    informe: Text;
    nomMae, nomDet, n: String;
    i: integer;

begin
    nomMae:= 'maestro';

    assign(mae, 'maestro');
    assign(informe, 'informe.txt');

    for i:= 1 to 20 do begin
        Str(i, n);
        nomDet:= 'detalle' + n;
        assign(archivos[i], nomDet);
    end;
    procedimiento(mae, archivos, registros, informe);
end.