use core::panic;


pub struct Triangulo{
    ladoA :f32,
    ladoB :f32,
    ladoC :f32,
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
    pub fn new(ladoA: f32, ladoB: f32, ladoC: f32) -> Triangulo{
        if (ladoA + ladoB < ladoC) | (ladoA + ladoC < ladoB) | (ladoB + ladoC < ladoA){ panic!("Triangulo incorrecto")}
        else{Triangulo { ladoA, ladoB, ladoC }}
    }
    pub fn determinarTipo(&self) -> Tipo{
        let mut AB = false;
        let mut AC = false;
        let mut BC = false;
        if self.ladoA == self.ladoB{AB = true}
        if self.ladoA == self.ladoC{AC = true}
        if self.ladoB == self.ladoC{BC = true}

        if AB && BC {return Tipo::Equilatero}
        else if AB ^ BC ^ AC {return Tipo::Isoseles}
        else{return Tipo::Escaleno}
    }
    pub fn calcular_area(&self) -> f32{
        let s = (self.ladoA + self.ladoB + self.ladoC) / 2.00;
        (s * (s - self.ladoA) * (s - self.ladoB) * (s - self.ladoC)).sqrt()
    }
    pub fn calcular_perimetro(&self) -> f32{
        self.ladoA + self.ladoB + self.ladoC
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn trianguloCorrecto(){
        let equilatero = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(equilatero.ladoA, 1.00);
        assert_eq!(equilatero.ladoB, 1.00);
        assert_eq!(equilatero.ladoC, 1.00);
    }

    #[should_panic]
    #[test]
    fn trianguloIncorrecto(){
        let equilatero = Triangulo::new(1.00, 1.00, 3.00);
    }

    #[test]
    fn trianguloEquilatero(){
        let equilatero = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(equilatero.determinarTipo().ig(&Tipo::Equilatero), true);
    }

    #[test]
    fn trianguloEscaleno(){
        let escaleno = Triangulo::new(1.00, 2.00, 3.00);
        assert_eq!(escaleno.determinarTipo().ig(&Tipo::Escaleno), true);
    }

    #[test]
    fn triangulosIsoseles(){
        let isoselesA = Triangulo::new(1.00, 1.00, 2.00);
        let isoselesB = Triangulo::new(1.00, 2.00, 1.00);
        let isoselesC = Triangulo::new(2.00, 1.00, 1.00);
        assert_eq!(isoselesA.determinarTipo().ig(&Tipo::Isoseles), true);
        assert_eq!(isoselesB.determinarTipo().ig(&Tipo::Isoseles), true);
        assert_eq!(isoselesC.determinarTipo().ig(&Tipo::Isoseles), true);
    }

    #[test]
    fn areaTriangulo(){
        let triangulo = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!((triangulo.calcular_area() * 100.00).trunc() / 100.00, 0.43);
        let trianguloB = Triangulo::new(2.00, 2.00, 3.00);
        assert_eq!((trianguloB.calcular_area() * 100.00).trunc() / 100.00, 1.98);

    }

    #[test]
    fn perimetroTriangulo(){
        let triangulo = Triangulo::new(1.00, 1.00, 1.00);
        assert_eq!(triangulo.calcular_perimetro(), 3.00);
    }
}