const
    valor_alto = 9999;
    cant_suc = 30;
type
    Producto = record
        cod: integer; nom: String; desc: String; stockDis: integer; stockMin: integer; pre: double; end;

    Venta = record
        cod: integer; cant: integer; end;

    Maestro = file of Producto;
    Detalle = file of Venta;

    arregloDet = array[0..cant_suc - 1] of Detalle;
    arregloReg = array[0..cant_suc - 1] of Venta;  

procedure leer_d(var arcdet: Detalle; var dato: Venta);
begin
    if (not EOF(arcdet)) then read(arcdet, dato)
    else dato.cod := valor_alto;
end;

procedure leer_m(var arcmae: Maestro; var dato: Producto);
begin
    if (not EOF(arcmae)) then read(arcmae, dato)
    else dato.cod := valor_alto;
end;

procedure minimo(var arrd: arregloDet; var arrr: arregloReg; var min_venta: Venta);
var
    imin, i: integer;
begin
    min_venta.cod := valor_alto;
    for i := 0 to cant_suc - 1 do begin
        if arrr[i].cod < min_venta.cod then begin
            min_venta.cod := arrr[i].cod;
            imin := i;
        end;
    end;

    if (min_venta.cod <> valor_alto) then begin
        min_venta.cant := arrr[imin].cant;
        leer_d(arrd[imin], arrr[imin]);
    end;
end;

procedure actualizar_stock(var arcmae: Maestro);
var
    i, venta_actual, cant_ventas: integer;
    nomdet, codigo, ceros: String;
    venta_min: Venta;
    datomae: Producto;
    arrd: arregloDet;
    arrr: arregloReg;
begin
    for i:= 0 to 29 do begin
        Str(i, codigo);
        if i < 10 then ceros := '00'
        else if i < 100 then ceros := '0';
        nomdet := (ceros + codigo + 'sucursal');
        
        assign(arrd[i], nomdet);
        reset(arrd[i]);
        leer_d(arrd[i], arrr[i]);
    end;

    reset(arcmae);

    leer_m(arcmae, datomae);
    minimo(arrd, arrr, venta_min);


    while (venta_min.cod <> valor_alto) do begin
        venta_actual := venta_min.cod;
        cant_ventas := 0;
        while (venta_actual = venta_min.cod) do begin
            cant_ventas := cant_ventas + venta_min.cant;
            minimo(arrd, arrr, venta_min);
        end;
        while (datomae.cod <> venta_actual) do begin
            leer_m(arcmae, datomae);
        end;
        datomae.stockDis := datomae.stockDis - cant_ventas;
        seek(arcmae, filePos(arcmae) - 1);
        write(arcmae,datomae);
        leer_m(arcmae, datomae);
    end;
    for i:= 0 to cant_suc - 1 do close(arrd[i]);
    close(arcmae);
end;

procedure generar_informe(var arcmae: Maestro);
var
    dato: Producto;
    informe: Text;
    nominf: String;
begin
    nominf := 'informeSeparado.txt';
    assign(informe, nominf);
    reset(arcmae);
    rewrite(informe);

    leer_m(arcmae, dato);
    while (dato.cod <> valor_alto) do begin
        if (dato.stockDis < dato.stockMin) then begin
            writeln(informe, dato.nom);
            writeln(informe, dato.desc);
            writeln(informe, dato.stockDis, ' ', dato.pre:0:2);
        end;
        leer_m(arcmae, dato);
    end;
    close(arcmae);
    close(informe);
end;
var
    arcmae: Maestro;
    nommae: String;
    a: Detalle;
    d: Venta;
    m: Producto;
begin
    nommae := 'maestro';
    assign(arcmae, nommae);

    assign(a, '000sucursal');
    reset(a);
    reset(arcmae);

    d.cod := 100;
    d.cant := 6;
    m.cod := 100;
    m.nom := 'aaa';
    m.stockDis := 15;
    m.desc := 'aa aa';
    m.stockMin := 10;
    m.pre := 1000;
    write(arcmae, m);
    write(a, d);
    close(arcmae);
    close(a);
    
    actualizar_stock(arcmae);
    generar_informe(arcmae);

end.