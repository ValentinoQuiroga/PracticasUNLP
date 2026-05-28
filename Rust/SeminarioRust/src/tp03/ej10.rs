use std::collections::{HashMap, LinkedList, VecDeque};
use crate::tp03::ej03::Fecha;
struct Biblioteca{nombre: String, direccion: String, libros_disponibles: HashMap<u64, (u16, Libro)>, prestamos_efectuados: VecDeque<Prestamo>}
struct Libro{isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero}
struct Prestamo{libro: Libro, cliente: Cliente, vencimiento: Fecha, devolucion: Option<Fecha>, devuelto: bool}
struct Cliente{nombre: String, telefono: String, correo_e: String}
enum Genero{novela, infantil, tecnico, otros}

impl Biblioteca{
    fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u16{
        match self.libros_disponibles.get(&libro.isbn){
            Some(dato) => { return dato.0}
            None => return 0
        }
    }
    fn decrementar_cantidad_de_copias(&mut self, libro: &Libro){
        match self.libros_disponibles.get_mut(&libro.isbn){
            Some(dato) => { dato.0 -= 1}
            None => ()
        }
    }
    fn incrementar_cantidad_de_copias(&mut self, libro: &Libro){
        match self.libros_disponibles.get_mut(&libro.isbn){
            Some(dato) => { dato.0 += 1}
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
    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, vencimiento: Fecha, devolucion: Fecha) -> bool{
        if let Some(dato) = self.libros_disponibles.get(&libro.isbn){
            if (self.contar_prestamos_de_cliente(&cliente) < 5) &&  (dato.0 > 0){
                self.decrementar_cantidad_de_copias(&libro);
                self.prestamos_efectuados.push_back(Prestamo::new(libro, cliente, vencimiento));
                true
            }else{false}
        }else{false}
    }
    fn ver_prestamos_a_vencer(&self, fecha: &Fecha, dias_restantes: u8) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        let mut fecha_limite: Fecha = fecha.clone();
        fecha_limite.sumar_dias(dias_restantes as u32);
        for i in 0..self.prestamos_efectuados.len(){
            let fecha_vencimiento: &Fecha = &self.prestamos_efectuados[i].vencimiento;
            let devuelto: bool = self.prestamos_efectuados[i].devuelto;
            if fecha.es_fecha_valida() && fecha_vencimiento.es_mayor(fecha) && fecha_limite.es_mayor(fecha_vencimiento) && !devuelto{
                lista.push(&self.prestamos_efectuados[i]);
            }
        }
        lista
    }
    fn ver_prestamos_vencidos(&self, fecha: &Fecha) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        for i in 0..self.prestamos_efectuados.len(){
            let fecha_vencimiento: &Fecha = &self.prestamos_efectuados[i].vencimiento;
            let devuelto: bool = self.prestamos_efectuados[i].devuelto;
            if fecha.es_fecha_valida() && fecha.es_mayor(fecha_vencimiento) && !devuelto{
                lista.push(&self.prestamos_efectuados[i]);
            }
        }
        lista
    }
    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo>{
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while pos < self.prestamos_efectuados.len() && !encontrado{
            let prestamo: &Prestamo = &self.prestamos_efectuados[pos];
            if prestamo.libro.ig(libro) && prestamo.cliente.ig(cliente){
                encontrado = true;
                return Some(prestamo);
            }else{ pos += 1;}
        }
        return None
    }
    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha: &Fecha){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while pos < self.prestamos_efectuados.len() && !encontrado{
            let mut prestamo: &mut Prestamo = &mut self.prestamos_efectuados[pos];
            if prestamo.libro.ig(libro) && prestamo.cliente.ig(cliente){
                prestamo.devuelto = true;
                prestamo.devolucion = Some(fecha.clone());
                encontrado = true;
            }else{ pos += 1;}
        }

        if encontrado{self.incrementar_cantidad_de_copias(libro);}
    }


}
impl Cliente{
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
    fn ig(&self, otro_libro: &Libro) -> bool{
        if (self.autor != otro_libro.autor)||(self.genero.ig(&otro_libro.genero))||(self.isbn != otro_libro.isbn)||(self.paginas != otro_libro.paginas)||(self.titulo != otro_libro.titulo){
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