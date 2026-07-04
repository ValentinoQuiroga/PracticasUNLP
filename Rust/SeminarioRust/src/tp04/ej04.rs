use std::collections::{HashMap, VecDeque};

use crate::tp03::ej3::Fecha;

struct Sistema{
    descuentos_por_categoria: HashMap<String, u8>,
    descuento_por_newsletter: u8,
    ventas_realizadas: VecDeque<Venta>
}
#[derive(Debug, Clone, PartialEq)]
struct Producto{
    nombre: String,
    categoria: String,
    precio_base: f64,
}
#[derive(Debug, Clone, PartialEq)]
struct Cliente{
    nombre: String,
    apellido: String,
    dni: u64,
    email: Option<String>,
    suscripcion: bool
}
#[derive(Debug, Clone, PartialEq)]
struct Vendedor{
    legajo: String,
    antiguedad: u8,
    salario: f64
}
#[derive(Debug, Clone, PartialEq)]
struct Venta{
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_de_pago: MedioDePago,
    productos: VecDeque<(u8, Producto)>
}

#[derive(Debug, Clone, PartialEq)]
enum MedioDePago{
    TarjetaDeCredito,
    TarjetaDeDebito,
    TransferenciaBancaria,
    Efectivo
}

impl Sistema{
    pub fn registrar_venta(&mut self, venta: Venta){
        self.ventas_realizadas.push_back(venta);
    }
    pub fn registro_por_categoria(&self) -> String{
        let mut registro = "Categoria -- Monto por categoria".to_string();
        let mut iter_v = self.ventas_realizadas.iter().clone();

        let mut aux: HashMap<String, f64> = HashMap::new();

        for v in iter_v{
            let mut iter_p = v.productos.clone().iter();
            for (c,p) in &v.productos{
                *aux.entry(p.categoria.clone()).or_insert(0.0) += *c as f64 * p.precio_base;
            }
        }

        for r in aux{
            registro = format!("{}{}{}{}{}",registro, r.0, " --     ", r.1, "\n");
        }
        registro
    }
    pub fn registro_por_medio_de_pago(&self) -> String{
        let mut registro = "Medio de pago -- Monto por medio".to_string();
        let mut iter_v = self.ventas_realizadas.iter().clone();

        let mut aux: HashMap<String, f64> = HashMap::new();
        let mut clave: String;

        for v in iter_v{
            clave = format!("{:?}", v.medio_de_pago);
            *aux.entry(clave.clone()).or_insert(0.0) += v.calcular_precio_final(self);
        }

        for r in aux{
            registro = format!("{}{}{}{}{}",registro, r.0, " --     ", r.1, "\n");
        }
        registro
    }
}
impl Venta{
    pub fn new(fecha: Fecha, cliente: Cliente, vendedor: Vendedor, 
        medio_de_pago: MedioDePago, productos: VecDeque<(u8, Producto)>) -> Venta{
        Venta { fecha, cliente, vendedor, medio_de_pago, productos }
    }
    pub fn calcular_precio_final(&self, sistema: &Sistema) -> f64{
        let mut total = 0.0;
        let mut iter_v = self.productos.iter().clone();

        for v in iter_v{
            let mut descuento = 0;
            match sistema.descuentos_por_categoria.get(&v.1.categoria){
                Some(d) => (descuento = *d),
                None => ()
            }
            total += v.0 as f64 * (v.1.precio_base - (v.1.precio_base * (descuento / 100) as f64));
        }
        total - (total * (sistema.descuento_por_newsletter / 100) as f64)
    }
    
}