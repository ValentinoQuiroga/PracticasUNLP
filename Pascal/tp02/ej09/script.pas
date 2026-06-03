const
    valor_alto = 9999;
type
    Fecha = record
        dd:integer;mm:integer;aaaa:integer;end;
    Cliente = record
        cod:integer;nom:String;ape:String;end;
    Venta = record
        cli:Cliente;fec:Fecha;mon:double;end;
    
    Maestro = file of Venta;

procedure leer_m(var mae: Maestro; var dato: Venta);
begin
    if (not EOF(mae)) then read(mae,dato)
    else dato.cli.cod := valor_alto;
end;
procedure informar_reporte(var mae: Maestro);
var
    totalDeEmpresa, montoAnual, montoMes: double;
    cliente_actual, mesActual, aaaaActual: integer;
    dato: Venta;
begin
    totalDeEmpresa := 0;
    leer_m(mae, dato);
    while(dato.cli.cod <> valor_alto) do begin
        cliente_actual := dato.cli.cod;
        writeln(dato.cli.cod, '/', dato.cli.nom, '/', dato.cli.ape);
        while (dato.cli.cod = cliente_actual) do begin
            montoAnual := 0;
            aaaaActual := dato.fec.aaaa;
            while (dato.fec.aaaa = aaaaActual) do begin
                montoMes := 0;
                mesActual := dato.fec.mm;
                while(dato.fec.mm = mesActual) do begin
                    montoMes := montoMes + dato.mon;
                    leer_m(mae, dato);
                end;
                if (montoMes > 0) then begin
                    writeln('Mes ', mesActual, ' - ', montoMes:0:2);
                    montoAnual:= montoAnual + montoMes;
                end;
            end;
            writeln('Año ', aaaaActual, ' - ', montoAnual:0:2);
            totalDeEmpresa := totalDeEmpresa + montoAnual;
        end;
    end;
    writeln('Total de la empresa: ', totalDeEmpresa:0:2);
end;

var
    mae: Maestro;
    nomMae: String;
begin
    nomMae := 'maestro';
    assign(mae, nomMae);
    reset(mae);
    informar_reporte(mae);
    close(mae);
end.