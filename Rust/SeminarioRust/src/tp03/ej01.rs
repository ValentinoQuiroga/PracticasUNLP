
struct Persona{
    nombre: String,
    edad: int,
    direccion: Optional<String>
}

impl Persona{
    fn new(nombre: String, edad:int, direccion: Optional<String>) -> Persona{
        Persona{nombre,edad,direccion}
    }
}


#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn crearPersonaCorrectamente(){
        let persona1 = Persona.new("Valentino", 24);
        assert_eq(persona1.nombre, "Valentino");
        assert_eq(persona1.edad, 24);
        assert_eq(persona1.direccion, None); 
    }
}