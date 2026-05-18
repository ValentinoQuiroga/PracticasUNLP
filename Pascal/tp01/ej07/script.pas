program ej07;

const
    valor_alto = 9999;
type
    novela = record
        cod: integer;
        nom: String;
        gen: String;
        pre: LongInt;
    end;

    arcNov = file of novela;

procedure menu();
begin
	writeln('------Menu------');
	writeln('0: cerrar');
	writeln('1: crear binario desde novelas.txt');
	writeln('2: agregar novela a binario');
	writeln('3: modificar novela en binario');
end;

procedure leerTexto(var texto: Text; var dato:novela);
begin
    if (not EOF(texto)) then
    begin
        readln(texto, dato.cod, dato.pre, dato.gen);
        readln(texto, dato.nom);
    end
    else
    begin
        dato.cod := valor_alto;
    end;
end;
procedure leerBinario(var binario: arcNov; var dato:novela);
begin
    if (not EOF(binario)) then
    begin
        read(binario,dato);
    end
    else
    begin
        dato.cod := valor_alto;
    end;
end;

procedure opcion1(var texto: Text);
var
    dato: novela;
    binario: arcNov;
    nomBinario: String;
begin

    writeln('Ingrese nombre del binario a crear: ');
    readln(nomBinario);

    assign(binario, nomBinario);
    rewrite(binario);
    reset(texto);

    leerTexto(texto, dato);
    while(dato.cod <> valor_alto) do
    begin
        write(binario, dato);
    end;
end;

procedure crearDato(var dato: novela);
begin
    writeln('Ingrese el codigo: ');
	readln(dato.cod);
    writeln('Ingrese el precio: ');
	readln(dato.pre);
    writeln('Ingrese el genero: ');
	readln(dato.gen);
    dato.gen := (' ' + dato.gen);
    writeln('Ingrese el nombre: ');
	readln(dato.nom);
end;

procedure opcion2();
var
    binario: arcNov;
    nomBinario: String;
    dato: novela;
begin
    writeln('Ingrese nombre del binario a crear: ');
    readln(nomBinario);

    assign(binario, nomBinario);
    reset(binario);

    crearDato(dato);

    seek(binario, fileSize(binario));
    write(binario, dato);
    close(binario);
end;

procedure opcion3();
var
	binarioNom: String;
	binario: arcNov;
	dato: novela;
	codigoIngresado: integer;
begin
	writeln('Ingrese nombre del binario');
	readln(binarioNom);
	assign(binario, binarioNom);
	reset(binario);

	writeln('Ingrese el codigo de la novela');
	readln(codigoIngresado);

	leerBinario(binario, dato);
	while (dato.cod <> valor_alto) and (dato.cod <> codigoIngresado) do
	begin
		leerBinario(binario,dato);
	end;

	if (dato.cod = codigoIngresado) then
	begin
		crearDato(dato);
		seek(binario, filePos(binario) - 1);
		write(binario, dato);
	end;

	close(binario);
end;

var
    opcion: integer;
    texto: Text;
    textoNom: String;
Begin

    textoNom := 'novelas.txt';
    assign(texto, textoNom);

    opcion:= -1;
    while (opcion <> 0) do
	begin
        menu();
        writeln('Ingrese opcion');
        readln(opcion);
        case (opcion) of
            1: opcion1(texto);
            2: opcion2();
            3: opcion3();
        end;
    end;
end.
