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
        let mut texto: String = self.nombre.to_string() + " " + &self.edad.to_string();
        match &self.direccion{
            Some(valor) => {texto = texto + " " + &valor},
            _ => ()
        }
        texto 
    }

    pub fn obtener_edad(&self) -> u32{
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: String){
        self.direccion = Some(nueva_direccion);
    }
}


#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn crearPersonaSinDireccion(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        assert_eq!(persona1.nombre, "Valentino");
        assert_eq!(persona1.edad, 24);
        assert_eq!(persona1.direccion, None); 
    }
    #[should_panic]
    #[test]
    fn panicDireccionVacia(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        assert_eq!(persona1.direccion.unwrap(), ""); 
    }
    #[test]
    fn crearPersonaConDireecion(){
        let persona1 = Persona::new("Valentino".to_string(), 24, Some("Calle 10".to_string()));
        assert_eq!(persona1.nombre, "Valentino");
        assert_eq!(persona1.edad, 24);
        assert_eq!(persona1.direccion, Some("Calle 10".to_string())); 
    }

    #[test]
    fn crearString(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        let texto: &str = "Valentino 24";
        assert_eq!(persona1.to_string(), texto);
    }

    #[test]
    fn pedirEdad(){
        let persona1 = Persona::new("Valentino".to_string(), 24, None);
        assert_eq!(persona1.obtener_edad(), 24);

    }

    #[test]
    fn cambiarDireccion(){
        let mut persona1 = Persona::new("Valentino".to_string(), 24, None);
        let texto: &str = "Valentino 24";
        assert_eq!(persona1.to_string(), texto);

        persona1.actualizar_direccion("Direccion nueva".to_string());
        let textoB: &str = "Valentino 24 Direccion nueva";
        assert_eq!(persona1.to_string(), textoB)

    }
}