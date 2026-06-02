const
    valor_alto = 9999;
type
    Producto = record
        cod: integer; nom: String; pre: double; stockActual: integer; stockMinimo: integer; end;

    Venta = record
        cod: integer; cantUnidades: integer;
    end;

    Maestro = file of Producto;
    Detalle = file of Venta;

procedure leer_m(var arcMae: Maestro; var dato: Producto);
begin
    if (not EOF(arcMae)) then begin
        read(arcMae, dato);
    end
    else
    begin
        dato.cod := valor_alto;
    end;
end;

procedure leer_d(var arcDet: Detalle; var dato: Venta);
begin
    if (not EOF(arcDet)) then begin
        read(arcDet, dato);
    end
    else
    begin
        dato.cod := valor_alto;
    end;
end;

procedure actualizar_maestro(var arcMae: Maestro; var arcDet: Detalle);
var
    datoMae: Producto;
    datoDet: Venta;
    ventaAct, unidadesAct: integer;
begin
    leer_d(arcDet, datoDet);
    leer_m(arcMae, datoMae);
    while (datoDet.cod <> valor_alto) do begin
        ventaAct := datoDet.cod;
        unidadesAct := 0;

        while (ventaAct = datoDet.cod) do begin
            unidadesAct := datoDet.cantUnidades + unidadesAct;
            leer_d(arcDet, datoDet);
        end;

        while (ventaAct <> datoMae.cod) do
            leer_m(arcMae, datoMae);
        
        datoMae.stockActual := datoMae.stockActual - unidadesAct;
        seek(arcMae, filePos(arcMae) - 1);
        write(arcMae, datoMae);
        if (not EOF(arcMae)) then leer_m(arcMae, datoMae);
    end;
end;

procedure generar_stock_min(var arcMae: Maestro);
var
    arcTxt: Text;
    dato: Producto;
begin
    assign(arcTxt, 'stock_minimo.txt');
    rewrite(arcTxt);
    reser(arcMae);

    leer_m(arcMae, dato);
    while (dato.cod <> valor_alto) do begin
        if (dato.stockActual < dato.stockMinimo) then begin
            writeln(arcTxt, dato.cod, ' ', dato.nom);
            writeln(arcTxt, dato.pre, ' ', dato.stockActual, ' ', dato.stockMinimo);
        end;
        leer_m(arcMae, dato);
    end;
    close(arcTxt);
end;

var
    arcMae: Maestro;
    arcDet: Detalle;
    nomMae, nomDet: String;
begin
    nomMae := 'maestro';
    nomDet := 'detalle';
    assign(arcMae, nomMae);
    assign(arcDet, nomDet);

    reset(arcMae);
    reset(arcDet);

    actualizar_maestro(arcMae, arcDet);
    generar_stock_min(arcMae);
    close(arcMae);
    close(arcDet);
end.