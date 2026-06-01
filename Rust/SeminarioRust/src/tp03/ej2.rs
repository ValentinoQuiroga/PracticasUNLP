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
    fn crearRectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.ancho, 3);
        assert_eq!(rec.longitud, 5);
    }
    #[test]
    fn obtenerAreaRectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.calcular_area(), 15);
    }
    #[test]
    fn obtenerPerimetroRectangulo(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.calcular_perimetro(), 16);
    }
    #[test]
    fn rectanguloCuadrado(){
        let rec: Rectangulo = Rectangulo::new(5, 5);
        assert_eq!(rec.es_cuadrado(), true);
    }
    #[test]
    fn noEsRectanguloCuadrado(){
        let rec: Rectangulo = Rectangulo::new(5, 3);
        assert_eq!(rec.es_cuadrado(), false);
    }
    #[test]
    fn rectanguloCuadradoMax(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.es_cuadrado(), true);
    }




    #[should_panic]
    #[test]
    fn rectanguloMaximoOverflowArea(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.calcular_area(), max);
    }
    #[should_panic]
    #[test]
    fn rectanguloMaximoOverflowPerimetro(){
        let max = u32::MAX;
        let rec: Rectangulo = Rectangulo::new(max, max);
        assert_eq!(rec.calcular_perimetro(), max);
    }
}