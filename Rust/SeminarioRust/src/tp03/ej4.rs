use core::panic;

pub struct Triangulo{
    lado_a :f32,
    lado_b :f32,
    lado_c :f32,
}

enum Tipo{
    Equilatero, Escaleno, Isoseles
}
impl Tipo{
    fn ig(&self, otro_tipo: &Tipo) -> bool{
        match (self, otro_tipo){
            (Tipo::Equilatero,Tipo::Equilatero) => true,
            (Tipo::Escaleno,Tipo::Escaleno) => true,
            (Tipo::Isoseles,Tipo::Isoseles) => true,
            _ => false
        }
    }
}
impl Triangulo{
    pub fn new(lado_a: f32, lado_b: f32, lado_c: f32) -> Triangulo{
        if (lado_a + lado_b < lado_c) | (lado_a + lado_c < lado_b) | (lado_b + lado_c < lado_a){ panic!("Triangulo incorrecto")}
        else{Triangulo { lado_a, lado_b, lado_c }}
    }
    pub fn determinar_tipo(&self) -> Tipo{
        let mut ab = false;
        let mut ac = false;
        let mut bc = false;
        if self.lado_a == self.lado_b{ab = true}
        if self.lado_a == self.lado_c{ac = true}
        if self.lado_b == self.lado_c{bc = true}

        if ab && bc {return Tipo::Equilatero}
        else if ab ^ bc ^ ac {return Tipo::Isoseles}
        else{return Tipo::Escaleno}
    }
    pub fn calcular_area(&self) -> f32{
        let s = (self.lado_a + self.lado_b + self.lado_c) / 2.00;
        (s * (s - self.lado_a) * (s - self.lado_b) * (s - self.lado_c)).sqrt()
    }
    pub fn calcular_perimetro(&self) -> f32{
        self.lado_a + self.lado_b + self.lado_c
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_triangulo_correcto(){
        let equilatero = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(equilatero.lado_a, 1.00);
        assert_eq!(equilatero.lado_b, 1.00);
        assert_eq!(equilatero.lado_c, 1.00);
    }

    #[should_panic]
    #[test]
    fn test_triangulo_incorrecto(){
        let equilatero = Triangulo::new(1.00, 1.00, 3.00);
    }

    #[test]
    fn test_triangulo_equilatero(){
        let equilatero = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(equilatero.determinar_tipo().ig(&Tipo::Equilatero), true);
    }

    #[test]
    fn test_triangulo_escaleno(){
        let escaleno = Triangulo::new(1.00, 2.00, 3.00);
        assert_eq!(escaleno.determinar_tipo().ig(&Tipo::Escaleno), true);
    }

    #[test]
    fn test_triangulos_isoseles(){
        let isoseles_a = Triangulo::new(1.00, 1.00, 2.00);
        let isoseles_b = Triangulo::new(1.00, 2.00, 1.00);
        let isoseles_c = Triangulo::new(2.00, 1.00, 1.00);
        assert_eq!(isoseles_a.determinar_tipo().ig(&Tipo::Isoseles), true);
        assert_eq!(isoseles_b.determinar_tipo().ig(&Tipo::Isoseles), true);
        assert_eq!(isoseles_c.determinar_tipo().ig(&Tipo::Isoseles), true);
    }

    #[test]
    fn test_area_triangulo(){
        let triangulo = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!((triangulo.calcular_area() * 100.00).trunc() / 100.00, 0.43);
        let triangulo_b = Triangulo::new(2.00, 2.00, 3.00);
        assert_eq!((triangulo_b.calcular_area() * 100.00).trunc() / 100.00, 1.98);

    }

    #[test]
    fn test_perimetro_triangulo(){
        let triangulo = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(triangulo.calcular_perimetro(), 3.00);
    }
}