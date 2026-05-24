pub struct Estudiante{
    nombre: String,
    id: u32,
    notas: Vec<Examen>
}

pub struct Examen{
    nombre: String,
    nota: f32
}

impl Examen{
    pub fn new(nombre: String, nota: f32) -> Examen{
        Examen{nombre, nota}
    }
}

impl Estudiante{
    pub fn new(nombre: String, id: u32) -> Estudiante{
        let notas: Vec<Examen> = Vec::new();
        Estudiante{nombre, id, notas}
    }
    pub fn obtener_promedio(&self) -> f32{
        let mut sumaTotal = 0.00;
        let cant = self.notas.len();
        if !self.notas.is_empty(){
            for i in 0..cant{
                sumaTotal += self.notas[i].nota;
            }
            return sumaTotal / cant as f32; 
        }else{ return 0.00}     
    }
    pub fn obtener_calificacion_mas_alta(&self) -> f32{
        let cant = self.notas.len();
        let mut max = 0.00;
        if !self.notas.is_empty(){
            for i in 0..cant{
                if self.notas[i].nota > max{ max = self.notas[i].nota}
            }
            return max
        }
        return -1.00
    }
    pub fn obtener_calificacion_mas_baja(&self) -> f32{
        let cant = self.notas.len();
        let mut min = 10.00;
        if !self.notas.is_empty(){
            for i in 0..cant{
                if self.notas[i].nota < min{ min = self.notas[i].nota}
            }
            return min
        }
        return -1.00
    }
}

#[cfg(test)]
mod tests{
    use super::*;
        #[test]
        fn promedio(){
            let ex1 = Examen::new("".to_string(), 10.00);
            let ex2 = Examen::new("".to_string(), 2.00);
            let ex3 = Examen::new("".to_string(), 3.00);
            let mut est = Estudiante::new("Valen".to_string(), 7);
            let mut est2 = Estudiante::new("Valen".to_string(), 7);
            est.notas.push(ex1);
            est.notas.push(ex2);
            est.notas.push(ex3);

            assert_eq!(est.obtener_promedio(), 5.00);
            assert_eq!(est2.obtener_promedio(), 0.00);
        }

        #[test]
        fn notaMasAlta(){
            let ex1 = Examen::new("".to_string(), 10.00);
            let ex2 = Examen::new("".to_string(), 2.00);
            let ex3 = Examen::new("".to_string(), 3.00);
            let mut est = Estudiante::new("Valen".to_string(), 7);
            let mut est2 = Estudiante::new("Valen".to_string(), 7);
            est.notas.push(ex1);
            est.notas.push(ex2);
            est.notas.push(ex3);

            assert_eq!(est.obtener_calificacion_mas_alta(), 10.00);
            assert_eq!(est2.obtener_calificacion_mas_alta(), -1.00);
        }

        #[test]
        fn notaMasBaja(){
            let ex1 = Examen::new("".to_string(), 10.00);
            let ex2 = Examen::new("".to_string(), 2.00);
            let ex3 = Examen::new("".to_string(), 3.00);
            let mut est = Estudiante::new("Valen".to_string(), 7);
            let mut est2 = Estudiante::new("Valen".to_string(), 7);
            est.notas.push(ex1);
            est.notas.push(ex2);
            est.notas.push(ex3);

            assert_eq!(est.obtener_calificacion_mas_baja(), 2.00);
            assert_eq!(est2.obtener_calificacion_mas_baja(), -1.00);
        }
}