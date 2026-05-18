program script;
const
	valor_alto = 9999;
type
	celulares = record
		cod: integer;
		nom: String;
		des: String;
		mar: String;
		pre: LongInt;
		stockMin: integer;
		stockDis: integer;
	end;

	archivo_celurares = file of celulares;

procedure menu();
begin
	writeln('------Menu------');
	writeln('0: cerrar');
	writeln('1: crear binario desde celulares.txt');
	writeln('2: listar celulares con stock menor al minimo');
	writeln('3: mostar celulares coincidientes con un fragmento de descripcion');
	writeln('4: exportar un binario a texto');
	writeln('5: agregar celular a un binario');
end;
procedure leer(var carga: Text; var dato:celulares);
begin
	if (not EOF(carga)) then
	begin
		readln(carga, dato.cod, dato.pre, dato.mar);
		readln(carga,dato.stockMin, dato.stockDis, dato.des);
		readln(carga,dato.nom);
	end
	else
	begin
		dato.cod := valor_alto;
	end;
end;

procedure leerBinario(var binario: archivo_celurares; var dato:celulares);
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
procedure imprimir( dato:celulares);
begin
	writeln(dato.cod, ' ', dato.pre, dato.mar);
	writeln(dato.stockMin, ' ', dato.stockDis, dato.des);
	writeln(dato.nom);
end;

procedure opcion1 (var carga: Text);
var
	registros: archivo_celurares;
	direccion: String;
	dato: celulares;
begin
	writeln('Ingrese la direccion del archivo a crear');
	readln(direccion);
	assign(registros,direccion);
	rewrite(registros);
	reset(carga);
	
	leer(carga, dato);
	while (dato.cod <> valor_alto) do
	begin
		write(registros, dato);
		leer(carga,dato);
	end;
	close(carga);
	close(registros);
end;

procedure opcion2(var carga: Text);
var
	dato: celulares;
begin
	reset(carga);
	leer(carga, dato);
	while (dato.cod <> valor_alto)do
	begin
		if (dato.stockDis > dato.stockMin) then imprimir(dato);
		leer(carga, dato);
	end;
end;

procedure opcion3(var carga:Text);
var
	dato: celulares;
	cadena: String;
begin
	reset(carga);
	writeln('Ingrese un fragmento de una descripcion: ');
	readln(cadena);

	leer(carga, dato);
	while(dato.cod <> valor_alto) do
	begin
		if (pos(UpCase(cadena), UpCase(dato.des)) > 0) then imprimir(dato);
		leer(carga, dato);
	end;
	close(carga);
end;

procedure opcion4(var carga: Text);
var
	binarioNom: String;
	binario: archivo_celurares;
	dato: celulares;
begin
	writeln('Ingrese nombre del binario');
	read(binarioNom);
	assign(binario, binarioNom);
	reset(binario);
	rewrite(carga);

	leerBinario(binario, dato);
	while (dato.cod <> valor_alto) do
	begin
		writeln(carga, dato.cod, ' ', dato.pre, ' ', dato.mar);
		writeln(carga, dato.stockDis, ' ', dato.stockMin, ' ', dato.des);
		writeln(carga, dato.nom);
		leerBinario(binario,dato);
	end;

	close(binario);
	close(carga);
end;

procedure opcion5();
var
	binarioNom: String;
	binario: archivo_celurares;
	dato: celulares;
begin
	writeln('Ingrese nombre del binario');
	readln(binarioNom);
	assign(binario, binarioNom);
	reset(binario);

	seek(binario, fileSize(binario));
	writeln('Ingrese el codigo: ');
	readln(dato.cod);
	writeln('Ingrese el precio: ');
	readln(dato.pre);
	writeln('Ingrese la marca: ');
	readln(dato.mar);
	writeln('Ingrese el stock minimo: ');
	readln(dato.stockMin);
	writeln('Ingrese el stock disponible: ');
	readln(dato.stockDis);
	writeln('Ingrese la descripcion: ');
	readln(dato.des);
	writeln('Ingrese el nombre: ');
	readln(dato.nom);
	write(binario, dato);
	close(binario);
end;

procedure opcion6();
var
	binarioNom: String;
	binario: archivo_celurares;
	dato: celulares;
	codigoIngresado: integer;
begin
	writeln('Ingrese nombre del binario');
	readln(binarioNom);
	assign(binario, binarioNom);
	reset(binario);

	writeln('Ingrese el codigo del celular a modificar');
	readln(codigoIngresado);

	leerBinario(binario, dato);
	while (dato.cod <> valor_alto) and (dato.cod <> codigoIngresado) do
	begin
		leerBinario(binario,dato);
	end;

	if (dato.cod = codigoIngresado) then
	begin
		writeln('Celular encontrado');
	end;
end;



Var
	carga: Text;
	opcion: integer;
Begin
	assign(carga,'celulares.txt');
	opcion := 9999;
	while (opcion <> 0) do
	begin
		menu();
		writeln('Ingrese opcion');
		readln(opcion);

		case (opcion) of
			1: opcion1(carga);
			2: opcion2(carga);
			3: opcion3(carga);
			4: opcion4(carga);
			5: opcion5();
			6: opcion6();
		end;
	end;
end.
