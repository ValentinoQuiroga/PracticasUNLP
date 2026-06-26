program GeneradorSimple;
type
    Aplicacion = record
        cod: integer; nom: String[50]; origen: String[50];
        precio: double; licDisp: integer; licMin: integer;
    end;

    Venta = record
        cod: integer; cant: integer;
    end;

    Maestro = file of Aplicacion;
    Detalle = file of Venta;

var
    mae: Maestro;
    det: Detalle;
    regM: Aplicacion;
    regD: Venta;
    i: integer;
    strI: string;
begin
    { 1. CREAR ARCHIVO MAESTRO (2 aplicaciones con 100 licencias c/u) }
    assign(mae, 'maestro');
    rewrite(mae);
    
    { Aplicación 10: Su precio es alto, va a superar los 10.000 con pocas ventas }
    regM.cod := 10; regM.nom := 'Sistemas Pro'; regM.origen := 'Argentina'; regM.precio := 3000.0; regM.licDisp := 100; regM.licMin := 5;
    write(mae, regM);
    
    { Aplicación 20: Su precio es bajo, no va a superar los 10.000 }
    regM.cod := 20; regM.nom := 'Antivirus Lite'; regM.origen := 'Brasil'; regM.precio := 4000.0; regM.licDisp := 100; regM.licMin := 10;
    write(mae, regM);
    
    close(mae);

    { 2. CREAR LOS 20 ARCHIVOS DETALLE }
    for i := 1 to 20 do begin
        Str(i, strI);
        assign(det, 'detalle' + strI);
        rewrite(det);
        
        { Solo le cargamos información al detalle 1 y al detalle 2 }
        if (i = 1) then begin
            { El Portal 1 vende 3 licencias de la app 10 (3 * 3000 = 9000) }
            regD.cod := 10; regD.cant := 3;
            write(det, regD);
            
            { Y vende 5 licencias de la app 20 }
            regD.cod := 20; regD.cant := 5;
            write(det, regD);
        end;
        
        if (i = 2) then begin
            { El Portal 2 vende 1 licencia más de la app 10 }
            { En total (Portal 1 + Portal 2) se venden 4 de la app 10 -> 4 * 3000 = 12.000 (Supera los 10k!) }
            regD.cod := 10; regD.cant := 1;
            write(det, regD);
            
            { Y vende 2 licencias de la app 20 }
            regD.cod := 20; regD.cant := 2;
            write(det, regD);
        end;
        
        { Si i de 3 a 20, el archivo queda vacío pero creado exitosamente }
        close(det);
    end;
    
    writeln('Archivos de prueba simples creados.');
    writeln('Maestro: Apps 10 y 20.');
    writeln('Detalles: 1 y 2 con info, del 3 al 20 vacios.');
end.