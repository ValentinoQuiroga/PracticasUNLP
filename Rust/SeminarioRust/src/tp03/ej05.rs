pub struct Producto{
    nombre: String,
    precio_bruto: f32,
    id: u32
}

impl Producto{
    fn new(nombre: String, precio_bruto: f32, id: u32) -> Producto{
        Producto{nombre, precio_bruto, id}
    }
    fn calcular_impuestos(&self, porcentaje: u32) -> f32{
        self.precio_bruto * porcentaje as f32 / 100.00
    }
    fn aplicar_descuento(&self, porcentaje: u32) -> f32{
        self.precio_bruto * porcentaje as f32 / 100.00
    }
    fn calcular_precio_total(&self, porcentaje_de_impuestos: Option<u32>, porcentaje_de_descuento: Option<u32>) -> f32{
        let mut impuestos: f32 = 0.00;
        let mut descuento: f32 = 0.00;
        if let Some(valor) = porcentaje_de_impuestos{ impuestos = self.calcular_impuestos(valor);}
        if let Some (valor) = porcentaje_de_descuento { descuento = self.aplicar_descuento(valor);}
        self.precio_bruto + impuestos - descuento
    }
}

#[cfg(test)]
mod tests{
    use super::*;
        #[test]
        fn impuestos(){
            let prod = Producto::new("Hojas".to_string(), 100.00, 1);
            assert_eq!(prod.calcular_impuestos(25), 25.00);
        }

        #[test]
        fn descuento(){
            let prod = Producto::new("Hojas".to_string(), 100.00, 1);
            assert_eq!(prod.aplicar_descuento(30), 30.00);
        }


        #[test]
        fn precioBrutoTotal(){
            let prod = Producto::new("Hojas".to_string(), 100.00, 1);
            assert_eq!(prod.calcular_precio_total(Some(30), Some(25)), 105.00);
        }
}