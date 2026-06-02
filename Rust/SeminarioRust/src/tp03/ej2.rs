use std::u32;

pub struct Rectangulo{
    longitud: u32,
    ancho: u32,
}

impl Rectangulo{
    pub fn new(longitud: u32, ancho: u32) -> Rectangulo{
        Rectangulo{longitud, ancho}
    }

    pub fn calcular_area(&self) -> u32{
        self.ancho * self.longitud
    }

    pub fn calcular_perimetro(&self) -> u32{
        (2 * self.ancho) + (2 * self.longitud)
    }

    pub fn es_cuadrado(&self) -> bool{
        self.ancho == self.longitud
    }
}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn test_crear_rectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.ancho, 3);
        assert_eq!(rec.longitud, 5);
    }
    #[test]
    fn test_obtener_area_rectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.calcular_area(), 15);
    }
    #[test]
    fn test_obtener_perimetro_rectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.calcular_perimetro(), 16);
    }
    #[test]
    fn test_rectangulo_cuadrado(){
        let rec: Rectangulo = Rectangulo::new(5, 5);
        assert_eq!(rec.es_cuadrado(), true);
    }
    #[test]
    fn test_no_es_rectangulo_cuadrado(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.es_cuadrado(), false);
    }
    #[test]
    fn test_rectangulo_cuadrado_max(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.es_cuadrado(), true);
    }




    #[should_panic]
    #[test]
    fn test_rectangulo_maximo_overflow_area(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.calcular_area(), max);
    }
    #[should_panic]
    #[test]
    fn test_rectangulo_maximo_overflow_perimetro(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.calcular_perimetro(), max);
    }
}