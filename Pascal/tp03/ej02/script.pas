const
    valor_alto = 9999;
type
    str_nom = String[30];
    str_des = String[100];

    Producto = record
        cod:integer;nom:str_nom;des:str_des;pre:double;stk:integer;end;
    
    Productos = file of Producto;

procedure leer_p(var prods: Productos; var dato:Producto);
begin
    if (not EOF(prods)) then read(prods, dato)
    else dato.cod := valor_alto;
end;
procedure dar_de_baja_prod_sin_stock(var prods: Productos);
var
    dato:Producto;
begin
    reset(prods);

    leer_p(prods, dato);
    while(dato.cod <> valor_alto) do begin
        if (dato.stk = 0) then begin
            dato.nom := ('?' + dato.nom);
            seek(prods, filePos(prods) - 1);
            write(prods, dato);
        end;
        leer_p(prods, dato);
    end;

    close(prods);
end;
var
    prods: Productos;
    nprods: String;
begin
    nprods:= 'productos';
    assign(prods,nprods);
    dar_de_baja_prod_sin_stock(prods);
end.