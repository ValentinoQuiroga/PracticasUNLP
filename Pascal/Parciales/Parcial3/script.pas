const
    VA = 9999;
type
    Fecha = record
        dd:integer; mm: integer; aaaa: integer;
    end;

    Sesion = record
        cod_usuario: integer; fecha: Fecha; tiempo_sesion: integer;
    end;

    Log = file of Sesion;
    arrayLogs = array[1..5] of Log;

procedure leer(var archivo: Log; var dato: Sesion);
begin
    if (not EOF(archivo)) then read(archivo, dato)
    else dato.cod_usuario:= VA;
end;

function fechas_iguales(a:Fecha; b:Fecha):boolean;
begin
    fechas_iguales:= ((a.dd = b.dd)and(a.mm = b.mm)and(a.aaaa = b.aaaa));
end;

procedure generar_archivo(var maestro: Log; var archivos: arrayLogs);
var
    auxiliar: Log;
    i, pos: integer;
    sesion_actual, dato: Sesion;
    fec_iguales: boolean;
begin
    reset(maestro);
    rewrite(auxiliar);
    for i:= 1 to 5 do begin
        reset(archivos[i]);
        pos:= 0;
        while(not EOF(archivos[i])) do begin
            leer(archivos[i], dato);
            sesion_actual := dato;
            sesion_actual.tiempo_sesion:= 0;
            while (dato.cod_usuario <> VA) do begin
                fec_iguales:= fechas_iguales(dato.fecha, sesion_actual.fecha);
                if (dato.cod_usuario = sesion_actual.cod_usuario) and (fec_iguales) then begin
                    sesion_actual.tiempo_sesion:= sesion_actual.tiempo_sesion + dato.tiempo_sesion;
                end;
                leer(archivos[i], dato);
            end;
            write(auxiliar, sesion_actual);
            pos:= pos + 1;
            seek(archivos[i], pos);
        end;
        close(archivos[i]);
    end;

    reset(auxiliar);
    pos:= 0;
    while(not EOF(auxiliar)) do begin
        leer(auxiliar, dato);
        sesion_actual := dato;
        sesion_actual.tiempo_sesion:= 0;
        while (dato.cod_usuario <> VA) do begin
            fec_iguales:= fechas_iguales(dato.fecha, sesion_actual.fecha);
            if (dato.cod_usuario = sesion_actual.cod_usuario) and (fec_iguales) then begin
                sesion_actual.tiempo_sesion:= sesion_actual.tiempo_sesion + dato.tiempo_sesion;
            end;
            leer(auxiliar,dato);
        end;
        write(maestro, sesion_actual);
        pos:= pos + 1;
        seek(auxiliar, pos);
    end;

    close(auxiliar);
    close(maestro);
end;

var
    maestro: Log;
    archivos: arrayLogs;
    i: integer;
    n, nomDet: String;
Begin
    assign(maestro, 'maestro');
    for i:= 1 to 5 do begin
        Str(i, n);
        nomDet:= 'detalle' + n;
        assign(archivos[i], nomDet);
    end;
    generar_archivo(maestro, archivos);
end.