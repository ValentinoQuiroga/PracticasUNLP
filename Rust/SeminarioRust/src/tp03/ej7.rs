use std::collections::{HashMap, VecDeque};

pub struct ConsecionarioAuto{
    nombre: String,
    direccion: String,
    capacidad_maxima: usize,
    autos: VecDeque<Auto>
}
#[derive(Clone)]

pub struct Auto{
    marca: String,
    modelo: String,
    aaaa: u16,
    precio_bruto: f64,
    color: Color
}
#[derive(Clone)]

pub enum Color{
    Rojo,
    Verde, 
    Azul, 
    Amarillo, 
    Blanco,
    Negro
}


    //STRUCT AGREGADO ENTREGABLE
pub struct Registro{
    autos_rojos: f64,
    autos_verdes: f64,
    autos_azules: f64,
    autos_amarillos: f64,
    autos_blancos: f64,
    autos_negros: f64,
}

impl Registro{
    pub fn new(autos_rojos: f64,autos_verdes: f64,autos_azules: f64,autos_amarillos: f64,autos_blancos: f64,autos_negros: f64,) -> Registro{
        Registro { autos_rojos, autos_verdes, autos_azules, autos_amarillos, autos_blancos, autos_negros }
    }
}
impl ConsecionarioAuto{
    //FUNCIONALIDAD AGREGADA ENTREGABLE (TEST REALIZADOS EN TESTS)
    pub fn recaudacion_por_color(&self) -> Registro{
        let mut registro_recaudacion: Registro = Registro::new(0.00, 0.00, 0.00, 0.00, 0.00, 0.00);
        for i in 0..self.autos.len(){
            let precio = self.autos[i].calcular_precio();
            match self.autos[i].color{
                Color::Rojo => registro_recaudacion.autos_rojos += precio,
                Color::Verde => registro_recaudacion.autos_verdes += precio,
                Color::Azul => registro_recaudacion.autos_azules += precio,
                Color::Amarillo => registro_recaudacion.autos_amarillos += precio,
                Color::Blanco => registro_recaudacion.autos_blancos += precio,
                Color::Negro => registro_recaudacion.autos_negros += precio,
            }
        }
        registro_recaudacion
    }


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
    pub fn new(marca: String, modelo: String, aaaa: u16, precio_bruto: f64, color: Color) -> Auto{
        if precio_bruto < 0.00{ panic!("Se ingreso un precio bruto negativo");}
        else{Auto{marca, modelo, aaaa, precio_bruto, color}}
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


    //TEST AGREGADO ENTREGABLE
    #[test]
    fn test_recaudacion_por_color_rojo_y_verde(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_rojo: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Rojo);
        let auto_verde: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Verde);
        assert_eq!(auto_rojo.calcular_precio(), 125000.00);
        assert_eq!(auto_verde.calcular_precio(), 90000.00);

        con.agregar_auto(auto_rojo.clone());
        con.agregar_auto(auto_rojo.clone());
        con.agregar_auto(auto_verde);
        

        let registro = con.recaudacion_por_color();
        assert_eq!(registro.autos_rojos, 250000.00);
        assert_eq!(registro.autos_verdes, 90000.00);
        assert_eq!(registro.autos_negros, 0.00);
        assert_eq!(registro.autos_blancos, 0.00);
        assert_eq!(registro.autos_amarillos, 0.00);
        assert_eq!(registro.autos_azules, 0.00);

    }


    //TEST AGREGADO ENTREGABLE
    #[test]
    fn test_recaudacion_por_color_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        
        let registro = con.recaudacion_por_color();
        assert_eq!(registro.autos_rojos, 0.00);
        assert_eq!(registro.autos_verdes, 0.00);
        assert_eq!(registro.autos_negros, 0.00);
        assert_eq!(registro.autos_blancos, 0.00);
        assert_eq!(registro.autos_amarillos, 0.00);
        assert_eq!(registro.autos_azules, 0.00);
    }


    #[test]
    #[should_panic(expected = "Se ingreso un precio bruto negativo")]
    fn test_crear_auto_precio_negativo(){
        let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, -1.00, Color::Negro);
    }
    #[test]
    fn test_agregar_auto_en_concensionario_con_espacio(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.agregar_auto(auto), true);
    }
    #[test]
    fn test_agregar_auto_en_concensionario_sin_espacio(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 1);
        let auto_a: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        let auto_b: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.agregar_auto(auto_a), true);
        assert_eq!(con.agregar_auto(auto_b), false);
    }
    #[test]
    fn test_eliminar_auto_en_concensionario_con_el_auto(){
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
    fn test_eliminar_auto_en_concensionario_sin_el_auto(){
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
    fn test_eliminar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a_eliminar: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        con.eliminar_auto(&auto_a_eliminar);
        assert_eq!(con.autos.len(), 0);
    }
    #[test]
    fn test_buscar_auto_en_concensionario_con_el_auto(){
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
    fn test_buscar_auto_en_concensionario_sin_el_auto(){
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
    fn test_buscar_auto_en_concensionario_sin_autos(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_a_buscar: Auto = Auto::new("D".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
        assert_eq!(con.autos.len(), 0);
        assert!(con.buscar_auto(&auto_a_buscar).is_none());
        assert_eq!(con.autos.len(), 0);
    }
    #[test]
    fn test_precio_auto_color_primario_y_auto_color_secundario(){
        let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
        let auto_primario: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Rojo);
        let auto_secundario: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Verde);
        assert_eq!(auto_primario.calcular_precio(), 125000.00);
        assert_eq!(auto_secundario.calcular_precio(), 90000.00);

        con.agregar_auto(auto_primario);
        con.agregar_auto(auto_secundario);
        

        let registro = con.recaudacion_por_color();
        assert_eq!(registro.autos_rojos, 125000.00);
        assert_eq!(registro.autos_verdes, 90000.00);
    }
    #[test]
    fn test_precio_auto_1999_2000_y_2001(){
        let auto99: Auto = Auto::new("A".to_string(), "A".to_string(), 1999, 100000.00, Color::Rojo);
        let auto00: Auto = Auto::new("A".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        let auto01: Auto = Auto::new("A".to_string(), "A".to_string(), 2001, 100000.00, Color::Rojo);
        assert_eq!(auto99.calcular_precio(), 120000.00);
        assert_eq!(auto00.calcular_precio(), 125000.00);
        assert_eq!(auto01.calcular_precio(), 125000.00);
    }
    #[test]
    fn test_precio_auto_bmw_y_no_bmw(){
        let auto_bmw: Auto = Auto::new("BMW".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        let auto_no_bmw: Auto = Auto::new("A".to_string(), "A".to_string(), 2000, 100000.00, Color::Rojo);
        assert_eq!(auto_bmw.calcular_precio(), 140000.00);
        assert_eq!(auto_no_bmw.calcular_precio(), 125000.00);
    }
}