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
        let mut precio: f64 = self.precio_bruto;

        match self.color{
            (Color::Rojo | Color::Amarillo | Color::Azul) => precio += precio * 25.00 / 100.00,
            _ => precio -= precio * 10.00 / 100.00
        }

        if self.marca == "BMW".to_string(){ precio += precio * 15.00 / 100.00}

        if self.aaaa < 2000{ precio -= precio * 5.00 / 100.00}

        precio 
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
        let autoA: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoB: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.agregar_auto(autoA), true);
        assert_eq!(con.agregar_auto(autoB), false);
    }
    #[test]
    fn eliminar_auto_en_concensionario_con_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoABuscar: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(auto);
        assert_eq!(con.autos.len(), 1);
        con.eliminar_auto(&autoABuscar);
        assert_eq!(con.autos.len(), 0);
        assert!(con.buscar_auto(&autoABuscar).is_none());
    }
    #[test]
    fn eliminar_auto_en_concensionario_sin_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let autoA: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoB: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoC: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoABuscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(autoA);
        con.agregar_auto(autoB);
        con.agregar_auto(autoC);
        assert_eq!(con.autos.len(), 3);
        con.eliminar_auto(&autoABuscar);
        assert_eq!(con.autos.len(), 3);
    }
    #[test]
    fn eliminar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let autoABuscar: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.eliminar_auto(&autoABuscar);
        assert_eq!(con.autos.len(), 0);
    }
    #[test]
    fn buscar_auto_en_concensionario_con_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let autoA: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoB: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoC: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoABuscar: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(autoA);
        con.agregar_auto(autoB);
        con.agregar_auto(autoC);
        assert_eq!(con.autos.len(), 3);
        assert_eq!(con.buscar_auto(&autoABuscar).unwrap().eq(&autoABuscar), true);
        assert_eq!(con.autos.len(), 2);
    }
    #[test]
    fn buscar_auto_en_concensionario_sin_el_auto(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let autoA: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoB: Auto = Auto::new("B".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoC: Auto = Auto::new("C".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let autoABuscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.agregar_auto(autoA);
        con.agregar_auto(autoB);
        con.agregar_auto(autoC);
        assert_eq!(con.autos.len(), 3);
        assert!(con.buscar_auto(&autoABuscar).is_none());
        assert_eq!(con.autos.len(), 3);
    }
    #[test]
    fn buscar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let autoABuscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.autos.len(), 0);
        assert!(con.buscar_auto(&autoABuscar).is_none());
        assert_eq!(con.autos.len(), 0);
    }
}