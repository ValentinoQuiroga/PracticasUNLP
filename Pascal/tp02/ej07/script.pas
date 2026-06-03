const
    valor_alto = 9999;
type
    Fecha = record
        dd:integer;mm:integer;aaaa:integer;end;
    Alumno = record
        cod:integer;ape:String;nom:String;cntCurApr:integer;cntFnlApr:integer;end;
    Cursada = record
        codA:integer;codM:integer;aaaa:integer;res:boolean;end;
    Final = record
        codA:integer;codM:integer;fec:Fecha;nota:double;end;

    Maestro = file of Alumno;
    Cursadas = file of Cursada;
    