const
    valor_alto = 9999;
type
    Producto = record
        cdgo:integer;nmbr:String;prec:double;skac:integer;skmn:integer;end;
    
    Venta = record
        cdgo:integer;cant:integer;end;

    Maestro = file of Producto;
    Detalle = file of Venta;

procedure leer_m(var mae:Maestro; var dato: Producto);
begin
    if (not EOF(mae)) then read(mae, dato)
    else dato.cdgo := valor_alto;
end;

procedure leer_d(var det:Detalle; var dato: Venta);
begin
    if (not EOF(det)) then read(det, dato)
    else dato.cdgo := valor_alto;
end;

procedure actualizar_maestro(var mae: Maestro);
var
    datoMae: Producto;
    det: Detalle;
    datoDet: Venta;
    nomDet: String;
begin
    writeln('Ingrese el nombre del archivo detalle: ');
    readln(nomDet);
    assign(det, nomDet);
    reset(det);
    reset(mae);

    leer_m(mae, datoMae);
    leer_d(det, datoDet);
    while(datoDet.cdgo <> valor_alto) do begin
        while(datoMae.cdgo <> valor_alto) and (datoMae.cdgo <> datoDet.cdgo) do begin
            leer_m(mae, datoMae);
        end;
        if (datoMae.cdgo = datoDet.cdgo) then begin
            datoMae.skac:= datoMae.skac - datoDet.cant;
            seek(mae, filePos(mae) - 1);
            write(mae, datoMae);
        end;
        reset(mae);
        leer_m(mae, datoMae);
        leer_d(det, datoDet);
    end;
    close(mae);
    close(det);
end;

var
    mae: Maestro;
    nomMae: String;
    det1, det2: Detalle;
    nomDet1, nomDet2: String;
begin
    {nomDet1:= 'det1';
    nomDet2:= 'det2';
    assign(det1, nomDet1);
    assign(det2, nomDet2);
    rewrite(det1);
    rewrite(det2);}
    nomMae:= 'maestro';
    assign(mae, nomMae);
    actualizar_maestro(mae);
end.