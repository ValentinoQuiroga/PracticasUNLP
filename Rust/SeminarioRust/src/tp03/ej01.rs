use std::ops::Add;


pub struct Persona{
    nombre: String,
    edad: u32,
    direccion: Option<String>
}

impl Persona{
    pub fn new(nombre: String, edad:u32, direccion: Option<String>) -> Persona{
        Persona{nombre,edad,direccion}
    }

    pub fn to_string(&self) -> String{
        let mut texto: String = self.nombre.to_string() + " " + &self.edad.to_string() + " ";
        match &self.direccion{
            Some(valor) => {texto = texto + &valor},
            _ => ()
        }
        texto 
    }
}


#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn crearPersonaCorrectamente(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        assert_eq!(persona1.nombre, "Valentino");
        assert_eq!(persona1.edad, 24);
        assert_eq!(persona1.direccion, None); 
    }

    #[test]
    fn crearString(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        println!("{}", persona1.to_string());
        let texto: &str = "Valentino24";
        assert_eq!(persona1.to_string(), texto);
    }
}