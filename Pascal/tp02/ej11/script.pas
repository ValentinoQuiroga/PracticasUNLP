const
    valor_alto = 9999;
type
    sub_rango = 1..15;
    Empleado = record 
        dpto:integer;divs:integer;num:integer;cat:sub_rango;hrs:integer;end;
    DatoCateg = record
        cat:integer;valor:double;end;
    Arreglo = array[1..15] of double;

    Empleados = file of Empleado;

procedure cargar_arreglo(var categorias: Text; var arrCat: Arreglo);
var
    dato:DatoCateg;
    i: integer;
begin
    reset(categorias);
    for i := 1 to 15 do begin
        readln(categorias, dato.cat, dato.valor);
        arrCat[i]:= dato.valor;
    end;
    close(categorias);
end;

procedure leer_e(var emp: Empleados; var dato: Empleado);
begin
    if (not EOF(emp)) then read(emp, dato)
    else dato.dpto:= valor_alto;
end;
procedure imprimir_listado(var emp: Empleados; arr: Arreglo);
var
    dato: Empleado;
    dptoAct, dptoHrs, divsAct, divsHrs: integer;
    dptoMnt, divsMnt, imp: double;
begin
    reset(emp);
    leer_e(emp, dato);
    while (dato.dpto <> valor_alto) do begin
        dptoAct:= dato.dpto;
        writeln(dptoAct);
        dptoHrs:= 0;
        dptoMnt:= 0;    
        while (dato.dpto = dptoAct) do begin
            divsAct:= dato.divs;
            writeln(divsAct);
            writeln('Numero de Empleado    Total de Hs.    Importe a cobrar');
            divsHrs:= 0;
            divsMnt:= 0;
            while (dato.dpto = dptoAct) and (dato.divs = divsAct) do begin
                imp := dato.hrs * arr[dato.cat];
                divsHrs:= divsHrs + dato.hrs;
                divsMnt:= divsMnt + imp;
                writeln('        ',dato.num,'        ','    ','     ',dato.hrs,'     ','    ','    ',imp:0:2,'    ');
                leer_e(emp, dato);
            end;
            writeln('   Total de horas division: ', divsHrs);
            writeln('   Monto total por division: ', divsMnt:0:2);
            dptoHrs:= dptoHrs + divsHrs;
            dptoMnt:= dptoMnt + divsMnt;    
        end;
        writeln('Total horas departamento: ', dptoHrs);    
        writeln('Monto total departamento: ', dptoMnt:0:2);    
        writeln('----------------------------------------------');
    end;
    close(emp);
end;
var
    arrCat: Arreglo;
    categorias: Text;
    nombreArreglo: String;
    emp: Empleados;
    nombreEmp: String; 
begin
    nombreEmp:= 'empleados';
    nombreArreglo:= 'categorias.txt';
    assign(emp, nombreEmp);
    assign(categorias, nombreArreglo);

    cargar_arreglo(categorias, arrCat);
    imprimir_listado(emp, arrCat);
end.