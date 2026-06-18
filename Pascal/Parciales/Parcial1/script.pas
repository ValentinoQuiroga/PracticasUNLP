const
    valor_alto = 9999;
type
    Registro = record
        arCod:integer;arNom:String[30];
        evCod:integer;evNom:String[30];
        aaaa:integer;
        likes:LongInt; dislikes:LongInt;puntj:double; 
    end;

    Archivo = file of Registro;


procedure leer(var arc: Archivo; var dato: Registro);
begin
    if (not EOF(arc)) then read(arc, dato)
    else dato.aaaa:= valor_alto;
end;

procedure actualizar_menos_influyente(aux: Registro; var menos: Registro);
begin
    if (aux.puntj < menos.puntj) then menos:= aux
    else if (aux.puntj = menos.puntj) and (aux.dislikes > menos.dislikes) then menos:= aux;
end;

procedure generar_informe(var arc: Archivo);
var
    dato: Registro;
    nroAaaa, nroPresentacionesTotales, aaaaActual, nroPresentaciones, eventoActual, artistaActual, likes, dislikes: integer;
    artista_menos_influyente, regAux: Registro;
    puntaje: double;
begin
    reset(arc);
    leer(arc, dato);

    writeln('Resumen de menor influencia por evento');
    nroAaaa:= 0;
    nroPresentacionesTotales:= 0;
    while(dato.aaaa <> valor_alto) do begin
        nroAaaa:= nroAaaa + 1;
        aaaaActual:= dato.aaaa;
        writeln('Año: ', dato.aaaa);
        nroPresentaciones:= 0;
        while(dato.aaaa = aaaaActual) do begin
            eventoActual:= dato.evCod;
            writeln('Evento: ', dato.evNom, ' (Codigo: ', dato.evCod, ')');
            artista_menos_influyente:= dato;
            artista_menos_influyente.puntj:= valor_alto;
            while(dato.aaaa <> valor_alto) and (dato.evCod = eventoActual)do begin
                regAux:= dato; 
                artistaActual:= dato.arCod;
                likes:= 0;
                dislikes:= 0;
                puntaje:= 0;
                writeln('Artista: ', dato.arNom, ' (Codigo: ', dato.arCod, ')');
                while(dato.evCod = eventoActual) and (dato.arCod = artistaActual) do begin
                    likes:= likes + dato.likes;
                    dislikes:= dislikes + dato.dislikes;
                    puntaje:= puntaje + dato.puntj;
                    nroPresentaciones:= nroPresentaciones + 1;
                    leer(arc, dato);
                end;
                regAux.dislikes:= dislikes;
                regAux.puntj:= puntaje;

                actualizar_menos_influyente(regAux, artista_menos_influyente);

                writeln('Likes totales: ', likes);
                writeln('Dislikes totales: ', dislikes);
                writeln('Direferencia: ', likes - dislikes);
                writeln('Puntaje total del jurado: ', puntaje);
            end;
            writeln('El artista ', artista_menos_influyente.arNom, ' fue el menos influyente de ', artista_menos_influyente.evNom, ' del año ', artista_menos_influyente.aaaa);
        end;
        nroPresentacionesTotales:= nroPresentacionesTotales + nroPresentaciones;
        writeln('Durante el año ', aaaaActual, ' se registraron ', nroPresentaciones, ' de presentaciones de artistas');
    end;
    if (nroAaaa > 0) then writeln('El numero total de presentaciones por año es de: ', (nroPresentacionesTotales/nroAaaa):0:2, ' presentaciones');
    close(arc);
end;

var
    arc: Archivo;
    nomArc: String;
begin
    nomArc:= 'archivo';
    assign(arc, nomArc);
    generar_informe(arc);
end.
