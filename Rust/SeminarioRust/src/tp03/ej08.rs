use std::collections::VecDeque;
pub struct Cancion{
    titulo: String,
    artista: String,
    genero: Genero
}

pub enum Genero{
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS
}

pub struct Playlist{
    lista: VecDeque<Cancion>,
    nombre: String
}

impl Genero{
    pub fn ig(&self, otro_genero: &Genero) -> bool{
        match (self, otro_genero){
            (Genero::ROCK, Genero::ROCK) =>true,
            (Genero::POP, Genero::POP) =>true,
            (Genero::RAP, Genero::RAP) =>true,
            (Genero::JAZZ, Genero::JAZZ) =>true,
            (Genero::OTROS, Genero::OTROS) =>true,
            _ => false
        }
    }
    pub fn clonar(&self) -> Genero{
        match self{
            Genero::ROCK => Genero::ROCK,
            Genero::POP => Genero::POP,
            Genero::RAP => Genero::RAP,
            Genero::JAZZ => Genero::JAZZ,
            Genero::OTROS => Genero::OTROS,
        }
    }
}


impl Cancion{
    pub fn new(titulo: String, artista: String, genero: Genero) -> Cancion{
        Cancion{titulo, artista, genero}
    }
    pub fn ig(&self, otra_cancion: &Cancion) -> bool{
        if (self.titulo != otra_cancion.titulo)||(self.artista != otra_cancion.artista)||!(self.genero.ig(&otra_cancion.genero)){
            return false
        }else{
            return true
        }
    }
    pub fn clonar(&self) -> Cancion{
        let titulo = self.titulo.clone();
        let artista = self.artista.clone();
        let genero = self.genero.clonar();
        Cancion{titulo, artista, genero}
    }
}
impl Playlist{
    pub fn new(nombre: String) -> Playlist{
        let lista: VecDeque<Cancion> = VecDeque::new();
        Playlist { lista, nombre }
    }
    pub fn agregar_cancion(&mut self, cancion: Cancion){
        self.lista.push_back(cancion);
    }
    pub fn eliminar_cancion(&mut self, cancion: &Cancion) -> Option<Cancion>{
        let cant: usize = self.lista.len();
        let mut i: usize = 0;
        let mut encontrado: bool = false;
        let mut cancionEliminada: Option<Cancion> = None;

        while (i < cant) && !(encontrado){
            if self.lista[i].ig(cancion){encontrado = true;
            }else{i += 1;}
        }
        if encontrado{ cancionEliminada = self.lista.remove(i);}
        return cancionEliminada
    }
    pub fn mover_cancion(&mut self, cancion: &Cancion, pos: usize){
        match self.eliminar_cancion(cancion) {
            Some(c) =>{
                if pos > self.lista.len() - 1{ self.lista.push_back(c);
                }else{ self.lista.insert(pos, c); }
            }
            None => ()
        }
    }
    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion>{
        let cant: usize = self.lista.len();
        let mut i: usize = 0;
        let mut encontrado: bool = false;

        while (i < cant) && !(encontrado){
            if self.lista[i].titulo == nombre{ encontrado = true;
            }else{ i += 1;}
        }

        match encontrado{
            true => Some(&self.lista[i]),
            false => None
        }

    }
    pub fn obtener_canciones_de_un_genero(&self, genero: &Genero) -> VecDeque<Cancion>{
        let mut lista: VecDeque<Cancion> = VecDeque::new();
        let cant: usize = self.lista.len();
        for i in 0..cant{
            let aux: Cancion = self.lista[i].clonar();
            if aux.genero.ig(genero){ lista.push_back(aux); }
        }
        lista
    }
    
    pub fn obtener_canciones_de_un_artista(&self, artista: String) -> VecDeque<Cancion>{
        let mut lista: VecDeque<Cancion> = VecDeque::new();
        let cant: usize = self.lista.len();
        for i in 0..cant{
            let aux: Cancion = self.lista[i].clonar();
            if aux.artista == artista{ lista.push_back(aux);}
        }
        lista
    }
    pub fn cambiar_nombre(&mut self, nombre_nuevo: String){
        self.nombre = nombre_nuevo;
    }
    pub fn limpiar_playlist(&mut self){
        self.lista.clear();
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn eliminar_cancion(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_a_eliminar: Cancion = can.clonar();
        p.agregar_cancion(can);
        assert_eq!(p.lista.len(), 1);
        p.eliminar_cancion(&can_a_eliminar);
        assert_eq!(p.lista.len(), 0);
    }
    #[test]
    fn eliminar_cancion_no_agregada(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_a_eliminar: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        p.agregar_cancion(can);
        assert_eq!(p.lista.len(), 1);
        p.eliminar_cancion(&can_a_eliminar);
        assert_eq!(p.lista.len(), 1);
    }
    #[test]
    fn mover_cancion(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("BC".to_string(), "B".to_string(), Genero::ROCK);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        assert_eq!(p.lista[0].ig(&can_a), true);
        p.mover_cancion(&can_a, 2);
        assert_eq!(p.lista[2].ig(&can_a), true);
    }
    #[test]
    fn mover_cancion_posicion_mayor_a_cantidad(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("BC".to_string(), "B".to_string(), Genero::ROCK);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        assert_eq!(p.lista[0].ig(&can_a), true);
        p.mover_cancion(&can_a, 20);
        assert_eq!(p.lista[2].ig(&can_a), true);
    }
    #[test]
    fn mover_cancion_no_agregada(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "B".to_string(), Genero::ROCK);
        let can_no_agregada: Cancion = Cancion::new("D".to_string(), "B".to_string(), Genero::ROCK);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        assert_eq!(p.lista[2].ig(&can_c), true);
        assert_eq!(p.lista.len(), 3);
        p.mover_cancion(&can_no_agregada, 20);
        assert_eq!(p.lista[2].ig(&can_c), true);
        assert_eq!(p.lista.len(), 3);
    }
    #[test]
    fn buscar_cancion(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        p.agregar_cancion(can.clonar());
        assert_eq!(p.buscar_cancion_por_nombre("A".to_string()).unwrap().ig(&can), true);
    }
    #[test]
    fn buscar_cancion_no_agregada(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        p.agregar_cancion(can);
        assert_eq!(p.buscar_cancion_por_nombre("B".to_string()).is_none(), true);
    }
    #[test]
    fn obtener_canciones_por_genero(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "B".to_string(), Genero::JAZZ);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        let nueva_lista: VecDeque<Cancion> = p.obtener_canciones_de_un_genero(&Genero::JAZZ);
        assert_eq!(nueva_lista.len(), 2);
        assert_eq!(nueva_lista[0].ig(&can_a), true);
        assert_eq!(nueva_lista[1].ig(&can_c), true);
    }
    #[test]
    fn obtener_canciones_por_genero_no_agregado(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "B".to_string(), Genero::JAZZ);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        let nueva_lista: VecDeque<Cancion> = p.obtener_canciones_de_un_genero(&Genero::ROCK);
        assert_eq!(nueva_lista.len(), 0);
        assert_eq!(nueva_lista.is_empty(), true);
    }
    #[test]
    fn obtener_canciones_por_artista(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "A".to_string(), Genero::JAZZ);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        let nueva_lista: VecDeque<Cancion> = p.obtener_canciones_de_un_artista("A".to_string());
        assert_eq!(nueva_lista.len(), 2);
        assert_eq!(nueva_lista[0].ig(&can_a), true);
        assert_eq!(nueva_lista[1].ig(&can_c), true);
    }
    #[test]
    fn obtener_canciones_por_artista_no_agregado(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "A".to_string(), Genero::JAZZ);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        let nueva_lista: VecDeque<Cancion> = p.obtener_canciones_de_un_artista("C".to_string());
        assert_eq!(nueva_lista.len(), 0);
        assert_eq!(nueva_lista.is_empty(), true);
    }
    #[test]
    fn cambiar_nombre_playlist(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        p.cambiar_nombre("Musica Jazz".to_string());
        assert_eq!(p.nombre, "Musica Jazz".to_string());
    }
    #[test]
    fn limpiar_playlist(){
        let mut p: Playlist = Playlist::new("Musica".to_string());
        let can_a: Cancion = Cancion::new("A".to_string(), "A".to_string(), Genero::JAZZ);
        let can_b: Cancion = Cancion::new("B".to_string(), "B".to_string(), Genero::POP);
        let can_c: Cancion = Cancion::new("C".to_string(), "A".to_string(), Genero::JAZZ);
        p.agregar_cancion(can_a.clonar());
        p.agregar_cancion(can_b.clonar());
        p.agregar_cancion(can_c.clonar());
        assert_eq!(p.lista.is_empty(), false);
        p.limpiar_playlist();
        assert_eq!(p.lista.is_empty(), true);
    }
}