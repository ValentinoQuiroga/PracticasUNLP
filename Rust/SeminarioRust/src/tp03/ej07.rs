use std::collections::VecDeque;

pub struct ConsecionarioAuto{
    nombre: String,
    direccion: String,
    capacidad_maxima: usize,
    autos: VecDeque<Auto>
}
pub struct Auto{
    marca: String,
    modelo: String,
    aaaa: u32,
    precio_bruto: f64,
    color: Color
}

pub enum Color{
    Rojo,
    Verde, 
    Azul, 
    Amarillo, 
    Blanco,
    Negro
}

impl ConsecionarioAuto{
    pub fn new(nombre: String, direccion: String, capacidad_maxima: usize) -> ConsecionarioAuto{
        let autos: VecDeque<Auto> = VecDeque::with_capacity(capacidad_maxima as usize);
        ConsecionarioAuto{nombre, direccion, capacidad_maxima, autos}
    }
    pub fn agregar_auto(&mut self, auto: Auto) -> bool{
        if self.autos.len() < self.capacidad_maxima as usize{
            self.autos.push_front(auto);
            return true
        }else{
            return false
        }
    }
    pub fn eliminar_auto(&mut self, auto: &Auto){
        let cant: usize = self.autos.len();
        let mut aux: Auto;

        for i in 0..cant{ 
            aux = self.autos.pop_front().unwrap();
            if !(aux.eq(&auto)){
                self.autos.push_back(aux);
            }
        }
    }
    pub fn buscar_auto(&mut self, auto: &Auto) -> Option<Auto>{
        let mut encontrado = false;
        let mut i: usize = 0;
        let mut aux: Option<Auto> = None;
        while i < self.autos.len() && !encontrado{
            match self.autos.front(){
                Some(auto_actual) => {
                    if auto_actual.eq(&auto){
                        encontrado = true;
                        aux = self.autos.pop_front();
                    }else{
                        let auto_incorrecto = self.autos.pop_front().unwrap();
                        self.autos.push_back(auto_incorrecto);
                    }
                }
                _ => ()
            }
            i += 1;
        }
        return aux;
    }
}

impl Auto{
    pub fn new(marca: String, modelo: String, aaaa: u32, precio_bruto: f64, color: Color) -> Auto{
        Auto{marca, modelo, aaaa, precio_bruto, color}
    }

    pub fn eq(&self, auto: &Auto) -> bool{
        if (self.marca != auto.marca) || (self.modelo != auto.modelo) || (self.aaaa != auto.aaaa) || (self.precio_bruto != auto.precio_bruto) || !(self.color.eq(&auto.color)){
            return false
        }else{ return true}
    }

    pub fn calcular_precio(&self) -> f64{
        let precio_base: f64 = self.precio_bruto;
        let mut precio_final: f64 = precio_base;

        match self.color{
            (Color::Rojo | Color::Amarillo | Color::Azul) => precio_final += precio_base * 25.00 / 100.00,
            _ => precio_final -= precio_base * 10.00 / 100.00
        }

        if self.marca == "BMW".to_string(){ precio_final += precio_base * 15.00 / 100.00}

        if self.aaaa < 2000{ precio_final -= precio_base * 5.00 / 100.00}

        precio_final 
    }
}

impl Color{
    pub fn eq(&self, color:&Color) -> bool{
        match (self, color){
            (Color::Rojo, Color::Rojo) => true,
            (Color::Verde, Color::Verde) => true,
            (Color::Azul, Color::Azul) => true,
            (Color::Amarillo, Color::Amarillo) => true,
            (Color::Blanco, Color::Blanco) => true,
            (Color::Negro, Color::Negro) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn agregar_auto_en_concensionario_con_espacio(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.agregar_auto(auto), true);
    }
    #[test]
    fn agregar_auto_en_concensionario_Sin_Espacio(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 1);
        let auto_a: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_b: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.agregar_auto(auto_a), true);
        assert_eq!(con.agregar_auto(auto_b), false);
    }
    #[test]
    fn eliminar_auto_en_concensionario_con_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_a_buscar: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(auto);
        assert_eq!(con.autos.len(), 1);
        con.eliminar_auto(&auto_a_buscar);
        assert_eq!(con.autos.len(), 0);
        assert!(con.buscar_auto(&auto_a_buscar).is_none());
    }
    #[test]
    fn eliminar_auto_en_concensionario_sin_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_b: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_c: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_a_eliminar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(auto_a);
        con.agregar_auto(auto_b);
        con.agregar_auto(auto_c);
        assert_eq!(con.autos.len(), 3);
        con.eliminar_auto(&auto_a_eliminar);
        assert_eq!(con.autos.len(), 3);
    }
    #[test]
    fn eliminar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a_eliminar: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.eliminar_auto(&auto_a_eliminar);
        assert_eq!(con.autos.len(), 0);
    }
    #[test]
    fn buscar_auto_en_concensionario_con_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_b: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_c: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_a_buscar: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(auto_a);
        con.agregar_auto(auto_b);
        con.agregar_auto(auto_c);
        assert_eq!(con.autos.len(), 3);
        assert_eq!(con.buscar_auto(&auto_a_buscar).unwrap().eq(&auto_a_buscar), true);
        assert_eq!(con.autos.len(), 2);
    }
    #[test]
    fn buscar_auto_en_concensionario_sin_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_b: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_c: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_a_buscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(auto_a);
        con.agregar_auto(auto_b);
        con.agregar_auto(auto_c);
        assert_eq!(con.autos.len(), 3);
        assert!(con.buscar_auto(&auto_a_buscar).is_none());
        assert_eq!(con.autos.len(), 3);
    }
    #[test]
    fn buscar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a_buscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.autos.len(), 0);
        assert!(con.buscar_auto(&auto_a_buscar).is_none());
        assert_eq!(con.autos.len(), 0);
    }
    #[test]
    fn precio_auto_color_primario_y_auto_color_secundario(){
        let auto_primario: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Rojo);
        let auto_secundario: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Verde);
        assert_eq!(auto_primario.calcular_precio(), 125000.00);
        assert_eq!(auto_secundario.calcular_precio(), 90000.00);
    }
    #[test]
    fn precio_auto_1999_2000_y_2001(){
        let auto99: Auto = Auto::new("A".to_string(), "A".to_string(), 1999, 100000.00, Color::Rojo);
        let auto00: Auto = Auto::new("A".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        let auto01: Auto = Auto::new("A".to_string(), "A".to_string(), 2001, 100000.00, Color::Rojo);
        assert_eq!(auto99.calcular_precio(), 120000.00);
        assert_eq!(auto00.calcular_precio(), 125000.00);
        assert_eq!(auto01.calcular_precio(), 125000.00);
    }
    #[test]
    fn precio_auto_bmw_y_no_bmw(){
        let auto_bmw: Auto = Auto::new("BMW".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        let auto_no_bmw: Auto = Auto::new("A".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        assert_eq!(auto_bmw.calcular_precio(), 140000.00);
        assert_eq!(auto_no_bmw.calcular_precio(), 125000.00);
    }
}