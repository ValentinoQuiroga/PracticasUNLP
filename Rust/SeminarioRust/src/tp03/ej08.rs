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
    pub fn eliminar_cancion(&mut self, cancion: &Cancion){
        let cant: usize = self.lista.len();
        let mut i: usize = 0;
        let mut encontrado: bool = false;

        while (i < cant) && !(encontrado){
            if self.lista[i].ig(cancion){
                encontrado = true;
            }else{
                i += 1;
            }
        }
        if encontrado{ self.lista.remove(i);}
    }
    pub fn mover_cancion(&mut self, cancion: Cancion, pos: usize){
        self.eliminar_cancion(&cancion);
        if pos > self.lista.len() - 1{
            self.lista.push_back(cancion);
        }else{
            self.lista.insert(pos, cancion);
        }
    }
    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion>{
        let cant: usize = self.lista.len();
        let mut i: usize = 0;
        let mut encontrado: bool = false;

        while (i < cant) && !(encontrado){
            if self.lista[i].titulo == nombre{
                encontrado = true;
            }else{
                i += 1;
            }
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
            if aux.genero.ig(genero){
                lista.push_back(aux);
            }
        }
        lista
    }
    
    pub fn obtener_canciones_de_un_artista(&self, artista: String) -> VecDeque<Cancion>{
        let mut lista: VecDeque<Cancion> = VecDeque::new();
        let cant: usize = self.lista.len();
        for i in 0..cant{
            let aux: Cancion = self.lista[i].clonar();
            if aux.artista == artista{
                lista.push_back(aux);
            }
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