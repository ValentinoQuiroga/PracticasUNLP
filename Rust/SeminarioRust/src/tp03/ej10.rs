use core::panic;
use std::collections::{HashMap, LinkedList, VecDeque};
use crate::tp03::ej03::Fecha;
struct Biblioteca{nombre: String, direccion: String, libros_disponibles: HashMap<u64, (u16, Libro)>, prestamos_efectuados: VecDeque<Prestamo>}
#[derive(Clone)]
struct Libro{isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero}
struct Prestamo{libro: Libro, cliente: Cliente, vencimiento: Fecha, devolucion: Option<Fecha>, devuelto: bool}
#[derive(Clone)]
struct Cliente{nombre: String, telefono: String, correo_e: String}
#[derive(Clone)]
enum Genero{novela, infantil, tecnico, otros}

impl Biblioteca{
    fn new(nombre: String, direccion: String) -> Biblioteca{
        let libros_disponibles: HashMap<u64, (u16, Libro)> = HashMap::new();
        let prestamos_efectuados: VecDeque<Prestamo> = VecDeque::new();
        Biblioteca { nombre, direccion, libros_disponibles, prestamos_efectuados }
    }
    fn agregar_libro(&mut self, libro: Libro){
        self.libros_disponibles.insert(libro.isbn, (1 as u16, libro));
    }
    fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u16{
        match self.libros_disponibles.get(&libro.isbn){
            Some(dato) => { return dato.0}
            None => return 0
        }
    }
    fn decrementar_cantidad_de_copias(&mut self, libro: &Libro){
        match self.libros_disponibles.get_mut(&libro.isbn){
            Some(dato) => { 
                if dato.0 > 0 {
                    dato.0 -= 1;
                }else{panic!("No hay mas copias disponibles de este libro")}
            }
            None => ()
        }
    }
    fn incrementar_cantidad_de_copias(&mut self, libro: &Libro){
        match self.libros_disponibles.get_mut(&libro.isbn){
            Some(dato) => { 
                if dato.0 < 65535{
                    dato.0 += 1
                }else{ panic!("Se alcanzo el maximo de libros")}}
            None => ()
        }
    }
    fn contar_prestamos_de_cliente(&self, cliente: &Cliente) -> u8{
        let mut cant: u8 = 0;
        for i in 0..self.prestamos_efectuados.len(){
            if self.prestamos_efectuados[i].cliente.ig(cliente) && !self.prestamos_efectuados[i].devuelto{ cant += 1}
        }
        cant
    }
    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, vencimiento: Fecha) -> bool{
        if let Some(dato) = self.libros_disponibles.get(&libro.isbn) && vencimiento.es_fecha_valida(){
            if (self.contar_prestamos_de_cliente(&cliente) < 5) &&  (dato.0 > 0){
                self.decrementar_cantidad_de_copias(&libro);
                self.prestamos_efectuados.push_front(Prestamo::new(libro, cliente, vencimiento));
                true
            }else{false}
        }else{false}
    }
    fn ver_prestamos_a_vencer(&self, fechaHoy: &Fecha, dias_restantes: u8) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        if fechaHoy.es_fecha_valida(){
            let mut pos: usize = 0;
            let mut detener_busqueda: bool = false;
            let mut fecha_limite: Fecha = fechaHoy.clone();
            fecha_limite.sumar_dias(dias_restantes as u32);

            while pos < self.prestamos_efectuados.len() && !detener_busqueda{
                let prestamo: &Prestamo = &self.prestamos_efectuados[pos];

                if prestamo.devuelto{detener_busqueda = true;}
                else{
                    let fecha_vencimiento: &Fecha = &self.prestamos_efectuados[pos].vencimiento;
                    if fecha_vencimiento.es_mayor(fechaHoy) && fecha_limite.es_mayor(fecha_vencimiento){
                        lista.push(&self.prestamos_efectuados[pos]);
                    }
                    pos += 1;
                }
            }
        }
        lista
    }
    fn ver_prestamos_vencidos(&self, fechaHoy: &Fecha) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        if fechaHoy.es_fecha_valida(){
            let mut pos: usize = 0;
            let mut detener_busqueda: bool = false;
            while pos < self.prestamos_efectuados.len() && !detener_busqueda{
                let prestamo: &Prestamo = &self.prestamos_efectuados[pos];

                if prestamo.devuelto{detener_busqueda = true;}
                else{
                    let fecha_vencimiento: &Fecha = &self.prestamos_efectuados[pos].vencimiento;
                    let devuelto: bool = self.prestamos_efectuados[pos].devuelto;
                    if fechaHoy.es_mayor(fecha_vencimiento){
                        lista.push(&self.prestamos_efectuados[pos]);
                    }
                    pos += 1;
                }
            }
        }
        lista
    }
    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo>{

        if self.libros_disponibles.contains_key(&libro.isbn){
            let mut pos: usize = 0;
            let mut encontrado: bool = false;

            while pos < self.prestamos_efectuados.len() && !encontrado{
                let prestamo: &Prestamo = &self.prestamos_efectuados[pos];
                if prestamo.libro.ig(libro) && prestamo.cliente.ig(cliente){
                    encontrado = true;
                    return Some(prestamo);
                }else{ pos += 1;}
            }
        }
        return None
    }
    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fechaHoy: Fecha){

        if self.libros_disponibles.contains_key(&libro.isbn) && fechaHoy.es_fecha_valida(){
            let mut pos: usize = 0;
            let mut encontrado: bool = false;
            let mut detener_busqueda: bool = false;
            while pos < self.prestamos_efectuados.len() && !encontrado && !detener_busqueda{
                let prestamo= &mut self.prestamos_efectuados[pos];
                if prestamo.devuelto{
                    detener_busqueda = true;
                }
                else if prestamo.libro.ig(libro) && prestamo.cliente.ig(cliente){
                    let mut prestamo = self.prestamos_efectuados.remove(pos).unwrap();
                    prestamo.devuelto = true;
                    prestamo.devolucion = Some(fechaHoy.clone());
                    encontrado = true;
                    self.prestamos_efectuados.push_back(prestamo);
                }else{ pos += 1;}
            }

            if encontrado{self.incrementar_cantidad_de_copias(libro);}
        }
    }


}
impl Cliente{
    fn new(nombre: String, telefono: String, correo_e: String) -> Cliente{
        Cliente { nombre, telefono, correo_e }
    }
    fn ig(&self, otro_cliente: &Cliente) -> bool{
        if (self.nombre != otro_cliente.nombre)||(self.telefono != otro_cliente.telefono)||(self.correo_e != otro_cliente.correo_e){
            return false
        }else{ return true}}
}
impl Prestamo{
    fn new(libro: Libro, cliente: Cliente, vencimiento: Fecha) -> Prestamo{
        Prestamo { libro, cliente, vencimiento, devolucion: None, devuelto: false }
    }
}
impl Libro{
    fn new(isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero) -> Libro{
        Libro { isbn, titulo, autor, paginas, genero }
    }
    fn ig(&self, otro_libro: &Libro) -> bool{
        if (self.autor != otro_libro.autor)||!(self.genero.ig(&otro_libro.genero))||(self.isbn != otro_libro.isbn)||(self.paginas != otro_libro.paginas)||(self.titulo != otro_libro.titulo){
            return false
        }else{ return true}}
}
impl Genero{
    fn ig(&self, otro_genero: &Genero) -> bool{
        match (self, otro_genero){
            (Genero::infantil, Genero::infantil) => true,
            (Genero::novela, Genero::novela) => true,
            (Genero::tecnico, Genero::tecnico) => true,
            (Genero::otros, Genero::otros) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_agregar_libro_nuevo(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());

        assert_eq!(bib.libros_disponibles.len(), 1);
        assert_eq!(bib.libros_disponibles.contains_key(&libro.isbn), true);
    }

    #[test]
    fn test_agregar_libro_ya_agregado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.agregar_libro(libro.clone());

        assert_eq!(bib.libros_disponibles.len(), 1);
        assert_eq!(bib.libros_disponibles.contains_key(&libro.isbn), true);
    }


    #[test]
    fn test_obtener_cantidad_de_copias_libro_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
    }

    #[test]
    fn test_obtener_cantidad_de_copias_libro_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 0);
    }


    #[test]
    fn test_decrementar_cantidad_de_copias_libro_registrado_con_una_copia(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.decrementar_cantidad_de_copias(&libro);

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 0);
    }
    #[test]
    #[should_panic(expected = "No hay mas copias disponibles de este libro")]
    fn test_decrementar_cantidad_de_copias_libro_registrado_sin_copia(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.decrementar_cantidad_de_copias(&libro);
        bib.decrementar_cantidad_de_copias(&libro);
    }
    #[test]
    fn test_decrementar_cantidad_de_copias_libro_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let libro_no_registrado: Libro = Libro::new(101, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.decrementar_cantidad_de_copias(&libro_no_registrado);

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
    }


    #[test]
    fn test_incrementar_cantidad_de_copias_libro_registrado_con_una_copia(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.incrementar_cantidad_de_copias(&libro);

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 2);
    }
    #[test]
    #[should_panic(expected = "Se alcanzo el maximo de libros")]
    fn test_incrementar_cantidad_de_copias_libro_registrado_con_maximo_de_copias(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        if let Some(dato) = bib.libros_disponibles.get_mut(&libro.isbn){
            dato.0 = 65535;
        }
        bib.incrementar_cantidad_de_copias(&libro);
    }
    #[test]
    fn test_incrementar_cantidad_de_copias_libro_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let libro_no_registrado: Libro = Libro::new(101, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.incrementar_cantidad_de_copias(&libro_no_registrado);

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
    }


    #[test]
    fn test_realizar_prestamo_libro_con_cinco_copias_cliente_con_cuatro_prestamos(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);


        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

        assert_eq!(bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone()),true);
        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 0);
        assert_eq!(bib.prestamos_efectuados.len(), 5);
    }

    #[test]
    fn test_realizar_prestamo_libro_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let libro_no_registrado: Libro = Libro::new(101, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());


        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);

        assert_eq!(bib.realizar_prestamo(libro_no_registrado.clone(), cliente.clone(), fecha_vencimiento.clone()), false);
        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
        assert_eq!(bib.prestamos_efectuados.len(), 0);
    }

    #[test]
    fn test_realizar_prestamo_libro_con_cero_copias(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.decrementar_cantidad_de_copias(&libro);


        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);

        assert_eq!(bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone()), false);
        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 0);
        assert_eq!(bib.prestamos_efectuados.len(), 0);
    }

    #[test]
    fn test_realizar_prestamo_cliente_con_cinco_prestamos(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);
        bib.incrementar_cantidad_de_copias(&libro);


        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

        assert_eq!(bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone()), false);
        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1); //Sobro una copia debido a que no se realizo un prestamo por le limite de 5 prestamos por cliente
        assert_eq!(bib.prestamos_efectuados.len(), 5);
    }

    #[test]
    fn test_realizar_prestamo_fecha_vencimiento_invalida(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(libro.clone());


        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento_invalida: Fecha = Fecha::new(100, 06, 2026);

        assert_eq!(bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento_invalida.clone()), false);
        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
        assert_eq!(bib.prestamos_efectuados.len(), 0);
    }


    #[test]
    fn test_devolver_libro_registrado_prestamo_vigente_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy: Fecha = Fecha::new(01, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.devolver_libro(&libro, &cliente, fecha_hoy.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
        assert_eq!(bib.prestamos_efectuados[0].devuelto, true);
    }

    #[test]
    fn test_devolver_libro_registrado_prestamo_ya_saldado_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy: Fecha = Fecha::new(01, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.devolver_libro(&libro, &cliente, fecha_hoy.clone());
        bib.devolver_libro(&libro, &cliente, fecha_hoy.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
        assert_eq!(bib.prestamos_efectuados[0].devuelto, true);
    }

    #[test]
    #[should_panic(expected = "Se alcanzo el maximo de libros")]
    fn test_devolver_libro_registrado_con_copias_maximas_disponibles(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy: Fecha = Fecha::new(01, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

        if let Some(dato) = bib.libros_disponibles.get_mut(&libro.isbn){
            dato.0 = 65535;
        }
        bib.devolver_libro(&libro, &cliente, fecha_hoy.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1);
        assert_eq!(bib.prestamos_efectuados[0].devuelto, true);
    }

    #[test]
    fn test_devolver_libro_registrado_prestamo_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy: Fecha = Fecha::new(01, 06, 2026);

        bib.agregar_libro(libro.clone());

        bib.devolver_libro(&libro, &cliente, fecha_hoy.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 1); //Se mantiene la copia en uno, ya que antes de incrementar las copias, devolver_libro corrobora la existencia del prestamo
    }

    #[test]
    fn test_devolver_libro_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let libro_no_registrado: Libro = Libro::new(101, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy: Fecha = Fecha::new(01, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.devolver_libro(&libro_no_registrado, &cliente, fecha_hoy.clone());

        assert_eq!(bib.libros_disponibles.contains_key(&libro_no_registrado.isbn), false); //no registra el libro devuelto
        assert_eq!(bib.prestamos_efectuados[0].devuelto, false) //el unico prestamo sigue vigente, ya que no se devolvio un libro valido
    }

    #[test]
    fn test_devolver_libro_fecha_invalida(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);
        let fecha_hoy_invalida: Fecha = Fecha::new(100, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
        bib.devolver_libro(&libro, &cliente, fecha_hoy_invalida.clone());

        assert_eq!(bib.obtener_cantidad_de_copias(&libro), 0); //no registra el libro devuelto por fecha actual invalida
    }

    
    #[test]
    fn test_contar_prestamos_cliente_no_registrado(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let cliente_no_registrado: Cliente = Cliente::new("A".to_string(), "A".to_string(), "B".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

        assert_eq!(bib.contar_prestamos_de_cliente(&cliente_no_registrado), 0);
    }

    #[test]
    fn test_contar_prestamos_cliente_sin_prestamos_pendientes(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let libro: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cliente: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let cliente_no_registrado: Cliente = Cliente::new("A".to_string(), "A".to_string(), "B".to_string());
        let fecha_vencimiento: Fecha = Fecha::new(20, 06, 2026);

        bib.agregar_libro(libro.clone());
        bib.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

        assert_eq!(bib.contar_prestamos_de_cliente(&cliente_no_registrado), 0);
    }

    #[test]
    fn test_aabb(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let l1: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let cli: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let ven: Fecha = Fecha::new(16, 04, 2027);
        let hoy: Fecha = Fecha::new(16, 04, 2028);
        bib.agregar_libro(l1.clone());
        bib.incrementar_cantidad_de_copias(&l1);
        bib.incrementar_cantidad_de_copias(&l1);
        bib.realizar_prestamo(l1.clone(), cli.clone(), ven.clone());
        bib.realizar_prestamo(l1.clone(), cli.clone(), ven.clone());
        bib.realizar_prestamo(l1.clone(), cli.clone(), ven.clone());

        bib.devolver_libro(&l1, &cli, hoy.clone());
        bib.devolver_libro(&l1, &cli, hoy.clone());

        assert_eq!(bib.prestamos_efectuados[0].devuelto, false);
        assert_eq!(bib.prestamos_efectuados[1].devuelto, true);
        assert_eq!(bib.prestamos_efectuados[2].devuelto, true);
    }
    #[test]
    fn test_obtener_cantidad_de_copias(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let l1: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(l1.clone());
        assert_eq!(bib.obtener_cantidad_de_copias(&l1), 1);
    }
    #[test]
    fn test_decrementar_cantidad_de_copias(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let l1: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(l1.clone());
        bib.decrementar_cantidad_de_copias(&l1);
        assert_eq!(bib.obtener_cantidad_de_copias(&l1), 0);
    }
    #[test]
    fn test_incrementar_cantidad_de_copias(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let l1: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(l1.clone());
        bib.incrementar_cantidad_de_copias(&l1);
        assert_eq!(bib.obtener_cantidad_de_copias(&l1), 2);
    }
    #[test]
    fn test_contar_cantidad_de_prestamos_de_cliente(){
        let mut bib: Biblioteca = Biblioteca::new("Biblio".to_string(), "10".to_string());
        let l1: Libro = Libro::new(100, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let l2: Libro = Libro::new(101, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let l3: Libro = Libro::new(102, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        let l4: Libro = Libro::new(103, "Titulo".to_string(), "A".to_string(), 100, Genero::novela);
        bib.agregar_libro(l1.clone());
        bib.agregar_libro(l2.clone());
        bib.agregar_libro(l3.clone());
        bib.agregar_libro(l4.clone());

        let cli: Cliente = Cliente::new("A".to_string(), "A".to_string(), "A".to_string());
        let cliB: Cliente = Cliente::new("A".to_string(), "A".to_string(), "B".to_string());
        let ven: Fecha = Fecha::new(16, 04, 2027);
        let vencido: Fecha = Fecha::new(10, 04, 2026);
        let act: Fecha = Fecha::new(10, 04, 2027);
        bib.realizar_prestamo(l1.clone(), cli.clone(), ven.clone());
        bib.realizar_prestamo(l2.clone(), cli.clone(), ven.clone());
        bib.realizar_prestamo(l3.clone(), cli.clone(), ven.clone());
        bib.realizar_prestamo(l4.clone(), cliB.clone(), vencido.clone());

        assert_eq!(bib.contar_prestamos_de_cliente(&cli),3);
        assert_eq!(bib.contar_prestamos_de_cliente(&cliB),1);
        assert_eq!(bib.obtener_cantidad_de_copias(&l1),0);

        bib.devolver_libro(&l1, &cli, act.clone());
        
        assert_eq!(bib.obtener_cantidad_de_copias(&l1),1);
        assert_eq!(bib.contar_prestamos_de_cliente(&cli),2);

        let a_vencer = bib.ver_prestamos_a_vencer(&act, 7);
        assert_eq!(a_vencer.len(), 2);
        let vencidos = bib.ver_prestamos_vencidos(&act);
        assert_eq!(vencidos.len(), 1);
    }
}