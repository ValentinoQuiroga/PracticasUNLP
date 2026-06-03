const
    valor_alto = 9999;
type
    Fecha = record
        dd:integer;mm:integer;aaaa:integer;end;
    Alumno = record
        cod:integer;ape:String;nom:String;cntCurApr:integer;cntFinApr:integer;end;
    Cursada = record
        codA:integer;codM:integer;aaaa:integer;apr:boolean;end;
    Final = record
        codA:integer;codM:integer;fec:Fecha;nota:double;end;

    Alumnos = file of Alumno;
    Cursadas = file of Cursada;
    Finales = file of Final;

procedure leer_c(var cur: Cursadas; var dato: Cursada);
begin
    if (not EOF(cur)) then read(cur, dato)
    else dato.codA := valor_alto
end;

procedure leer_f(var fin: Finales; var dato: Final);
begin
    if (not EOF(fin)) then read(fin, dato)
    else dato.codA := valor_alto
end;

procedure leer_a(var alm: Alumnos; var dato: Alumno);
begin
    if (not EOF(alm)) then read(alm, dato)
    else dato.cod := valor_alto
end;

procedure merge_cursadas(var rcur: Cursada; var cur: Cursadas; var cod: integer; var cant: integer);
begin
    cant := 0;
    while(cod = rcur.codA) do begin
        if (rcur.apr = true) then cant := cant + 1;
        leer_c(cur, rcur);
    end;
end;

procedure merge_finales(var rfin: Final; var fin: Finales; var cod: integer; var cant: integer);
begin
    cant := 0;
    while(cod = rfin.codA) do begin
        if (rfin.nota >= 4) then cant := cant + 1;
        leer_f(fin, rfin);
    end;
end;

procedure actualizar_maestro(var alm: Alumnos; var cur: Cursadas; var fin: Finales);
var
    rcur: Cursada;
    rfin: Final;
    ralm: Alumno;

    codact, cantf, cantc: integer;
begin
    reset(alm);
    reset(cur);
    reset(fin);

    leer_c(cur, rcur);
    leer_f(fin, rfin);
    leer_a(alm, ralm);

    while (rcur.codA <> valor_alto) or (rfin.codA <> valor_alto) do begin
        if (rcur.codA < rfin.codA) then begin
            codact := rcur.codA;
            merge_cursadas(rcur, cur, codact, cantc);
            cantf := 0;
        end
        else if (rcur.codA > rfin.codA) then begin
            codact := rfin.codA;
            merge_finales(rfin, fin, codact, cantf);
            cantc := 0;
        end
        else begin
            codact := rcur.codA;
            merge_cursadas(rcur, cur, codact, cantc);
            merge_finales(rfin, fin, codact, cantf);
        end;
        while (ralm.cod <> codact) do leer_a(alm,ralm);
        ralm.cntCurApr := ralm.cntCurApr + cantc;
        ralm.cntFinApr := ralm.cntFinApr + cantf;
        seek(alm, filePos(alm) - 1);
        write(alm, ralm);
        leer_a(alm, ralm);
    end;

    close(alm);
    close(cur);
    close(fin);
end;
var
    alm: Alumnos;
    cur: Cursadas;
    fin: Finales;
begin
    assign(alm, 'alumnos');
    assign(cur, 'cursadas');
    assign(fin, 'finales');
    
    actualizar_maestro(alm, cur, fin);
end.