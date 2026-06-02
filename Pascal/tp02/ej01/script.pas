const
    valor_alto = 9999;
type

    empleado = record
        cod: integer; nom: String; monto: double; end;
    
    registros = file of empleado;

procedure leerBinario(var binario: registros; var dato: empleado);
begin
    if (not EOF(binario)) then
    begin
        read(binario, dato);
    end
    else
    begin
        dato.cod := valor_alto;
    end;
end;

procedure compactar();
var
    nomBin, nomCom: String;
    arcBin, arcCom: registros;
    datoBin: empleado;
    empleadoActual: empleado;
begin
    writeln('Ingrese nombre del binario a compactar');
    readln(nomBin);
    nomCom := (nomBin + '_compacto');

    assign(arcBin, nomBin);
    assign(arcCom, nomCom);
    reset(arcBin);
    rewrite(arcCom);

    leerBinario(arcBin, datoBin);

    while (datoBin.cod <> valor_alto) do
    begin
        empleadoActual := datoBin;
        leerBinario(arcBin, datoBin);
        while (empleadoActual.cod = datoBin.cod) do
        begin
            empleadoActual.monto := empleadoActual.monto + datoBin.monto;
            leerBinario(arcBin, datoBin);
        end;
        write(arcCom, empleadoActual);
    end;
    close(arcBin);
    close(arcCom);
end;
var
    bin: registros;
begin
    assign(bin, 'test1');
    rewrite(bin);
    compactar();
end.