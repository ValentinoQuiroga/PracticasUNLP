use std::collections::{HashMap, LinkedList, VecDeque};
use crate::tp03::ej03::Fecha;
struct Biblioteca{nombre: String, direccion: String, libros_disponibles: HashMap<u64, (u16, Libro)>, prestamos_efectuados: HashMap<String, VecDeque<Prestamo>>}
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
        let prestamos_efectuados: HashMap<String, VecDeque<Prestamo>> = HashMap::new();
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
    fn decrementar_cantidad_de_copias(&mut self, isbn: u64){
        match self.libros_disponibles.get_mut(&isbn){
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
        let prestamos: Option<&VecDeque<Prestamo>> = self.prestamos_efectuados.get(&cliente.correo_e);
        match prestamos{
            Some(lista) => {
                for i in 0..lista.len(){
                    if !lista[i].devuelto{ cant += 1;}
                }
                cant
            }
            None => 0
        }
    }
    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, vencimiento: Fecha) -> bool{
        if let Some(dato) = self.libros_disponibles.get(&libro.isbn){ 
            if dato.0 > 0{
                if self.prestamos_efectuados.contains_key(&cliente.correo_e){
                    let cant = self.contar_prestamos_de_cliente(&cliente);
                    if cant > 5 {return false}
                }else{
                    let mut prestamos: VecDeque<Prestamo> = VecDeque::new();
                    self.prestamos_efectuados.insert(cliente.correo_e.clone(), prestamos);
                    
                }
                self.decrementar_cantidad_de_copias(libro.isbn);
                self.prestamos_efectuados.get_mut(&cliente.correo_e).unwrap().push_back(Prestamo::new(libro, cliente, vencimiento));
                return true
            }
        }
        false
    }
    fn ver_prestamos_a_vencer(&self, hoy: &Fecha, dias_restantes: u8) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        let mut fecha_limite: Fecha = hoy.clone();
        fecha_limite.sumar_dias((dias_restantes + 1) as u32);
        for (correo, prestamos) in &self.prestamos_efectuados{
            for i in 0..prestamos.len(){
                let fecha_vencimiento: &Fecha = &prestamos[i].vencimiento;
                let devuelto: bool = prestamos[i].devuelto;
                if hoy.es_fecha_valida() && fecha_vencimiento.es_mayor(hoy) && fecha_limite.es_mayor(fecha_vencimiento) && !devuelto{
                    lista.push(&prestamos[i]);
                }
            }
        }
        lista
    }
    fn ver_prestamos_vencidos(&self, fecha: &Fecha) -> Vec<&Prestamo>{
        let mut lista: Vec<&Prestamo> = Vec::new();
        for (correo, prestamos) in &self.prestamos_efectuados{
            for i in 0..prestamos.len(){
                let fecha_vencimiento: &Fecha = &prestamos[i].vencimiento;
                let devuelto: bool = prestamos[i].devuelto;
                if fecha.es_fecha_valida() && fecha.es_mayor(fecha_vencimiento) && !devuelto{
                    lista.push(&prestamos[i]);
                }
            }
        }
        lista
    }
    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo>{

        if let Some(prestamos) = self.prestamos_efectuados.get(&cliente.correo_e){
            let mut pos: usize = 0;
            let mut encontrado: bool = false;

            while pos < prestamos.len() && !encontrado{
                let prestamo: &Prestamo = &prestamos[pos];
                if prestamo.libro.ig(libro) && prestamo.cliente.ig(cliente){
                    encontrado = true;
                    return Some(prestamo);
                }else{ pos += 1;}
            }
        }
        
        return None
    }
    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha: Fecha){

        if let Some(prestamos) = self.prestamos_efectuados.get_mut(&cliente.correo_e){
            let mut pos: usize = 0;
            let mut encontrado: bool = false;

            while pos < prestamos.len() && !encontrado{
                let prestamo = &mut prestamos[pos];
                if prestamo.libro.ig(libro) && !prestamo.devuelto{
                    prestamo.devuelto = true;
                    prestamo.devolucion = Some(fecha.clone());
                    encontrado = true;
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
        bib.decrementar_cantidad_de_copias(l1.isbn);
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

        bib.devolver_libro(&l1, &cli, ven);
        
        assert_eq!(bib.obtener_cantidad_de_copias(&l1),1);
        assert_eq!(bib.contar_prestamos_de_cliente(&cli),2);

        let a_vencer = bib.ver_prestamos_a_vencer(&act, 7);
        assert_eq!(a_vencer.len(), 2);
        let vencidos = bib.ver_prestamos_vencidos(&act);
        assert_eq!(vencidos.len(), 1);
    }
}