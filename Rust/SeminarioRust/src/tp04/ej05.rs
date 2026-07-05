use std::collections::{HashMap, VecDeque};

use crate::tp03::ej3::Fecha;

#[derive(Debug, Clone)]
struct Plataforma{
    nombre: String,
    usuarios: HashMap<u64,Usuario>,
    registro: HashMap<u64, VecDeque<Transaccion>>,
    tabla_cotizacion: HashMap<String, VecDeque<(String, f64)>>
}
#[derive(Debug, Clone)]
struct Usuario{
    nombre: String,
    apellido: String,
    email: String,
    dni: u64,
    identidad_validada: bool,
    balance: HashMap<String, f64>,
}
#[derive(Debug, Clone)]
struct Transaccion{
    fecha: Fecha,
    tipo: String,
    monto: f64,
    usuario: Usuario
}
#[derive(Debug, Clone, PartialEq)]
struct Fiat{
    nombre: String,
    prefijo: String
}
#[derive(Debug, Clone, PartialEq)]
struct Criptomoneda{
    nombre: String,
    prefijo: String,
    blockchains: VecDeque<Blockchain>
}
#[derive(Debug, Clone, PartialEq)]
struct Blockchain{
    nombre: String,
    prefijo: String
}

impl Plataforma{
    pub fn new(nombre: String) -> Plataforma{
        let usuarios: HashMap<u64,Usuario> = HashMap::new();
        let registro: HashMap<u64, VecDeque<Transaccion>> = HashMap::new();
        let tabla_cotizacion: HashMap<String, VecDeque<(String, f64)>> = HashMap::new();
        Plataforma { nombre, usuarios, registro, tabla_cotizacion }
    }
    pub fn registrar_usuario(&mut self, usuario: Usuario){
        self.usuarios.insert(usuario.dni, usuario);
    }
    pub fn ingresar_dinero(&mut self, fecha: Fecha, monto_fiat: (Fiat, f64), usuario: &mut Usuario){
        let tipo: String = "(ingreso de dinero)".to_string();
        let fiat: String = monto_fiat.0.nombre;
        if self.usuarios.contains_key(&usuario.dni){
            *usuario.balance.entry(fiat.clone()).or_insert(0.0) += monto_fiat.1;
            self.registrar_transaccion(fecha, tipo, monto_fiat.1, &usuario);
        }else{panic!("Usuario no registrado")}
    }
    pub fn registrar_transaccion(&mut self, fecha: Fecha, tipo: String, monto: f64, usuario: &Usuario){
        let transaccion: Transaccion = Transaccion::new(fecha, tipo, monto, usuario.clone());
        self.registro.entry(usuario.dni).or_insert(VecDeque::new()).push_back(transaccion);
    }
    pub fn obtener_cotizacion(&self, nombre_a: &String, nombre_b: &String) -> f64{
        if let Some(lista) = self.tabla_cotizacion.clone().get_mut(nombre_a){
            for m in lista{
                if m.0.eq(nombre_b){
                    return m.1
                }
            }
        }
        panic!("Esta moneda no cuenta con una cotizacion para la moneda solicitada");
    }
}
impl Usuario{
    pub fn new(nombre: String, apellido: String, email: String, 
        dni: u64, identidad_validada: bool) -> Usuario{
        let balance: HashMap<String, f64> = HashMap::new();
        Usuario { nombre, apellido, email, dni, identidad_validada, balance }
    }
    pub fn comprar_criptomoneda(&mut self, fecha: Fecha, monto_en_fiat: (Fiat, f64), cripto: Criptomoneda,
                                                                                 sistema: &mut Plataforma){
        let fiat = monto_en_fiat.0.nombre;
        let monto = monto_en_fiat.1;
        if let Some(cantidad) = self.balance.get(&fiat){
            if *cantidad >= monto{
                let monto_final = monto * sistema.obtener_cotizacion(&fiat, &cripto.nombre);
                *self.balance.get_mut(&fiat).unwrap() -= monto;
                *self.balance.entry(cripto.nombre).or_insert(0.0) += monto_final;
                let tipo: String = "compra de cripto".to_string();
                sistema.registrar_transaccion(fecha, tipo, monto_final, self);
            }
        }else{ panic!("El usuario no cuenta con acceso a la moneda de ingreso")}
    }

    pub fn vender_criptomoneda(&mut self, fecha: Fecha, monto_en_cripto: (Criptomoneda, f64), fiat: Fiat,
                                                                                 sistema: &mut Plataforma){
        let cripto = monto_en_cripto.0.nombre;
        let monto = monto_en_cripto.1;
        if let Some(cantidad) = self.balance.get(&cripto){
            if *cantidad >= monto{
                let monto_final = monto * sistema.obtener_cotizacion(&cripto, &fiat.nombre);
                *self.balance.get_mut(&cripto).unwrap() -= monto;
                *self.balance.entry(fiat.nombre).or_insert(0.0) += monto_final;
                let tipo: String = "venta de cripto".to_string();
                sistema.registrar_transaccion(fecha, tipo, monto_final, self);
            }
        }else{ panic!("El usuario no cuenta con acceso a la moneda de ingreso")}
    }

}
impl Transaccion{
    pub fn new(fecha: Fecha, tipo: String, monto: f64, usuario: Usuario) -> Transaccion{
        Transaccion { fecha, tipo, monto, usuario }
    }
}
impl Fiat{
    pub fn new(nombre: String, prefijo: String) -> Fiat{
        Fiat { nombre, prefijo }
    }
}
impl Criptomoneda{
    pub fn new(nombre: String, prefijo: String) -> Criptomoneda{
        let blockchains: VecDeque<Blockchain> = VecDeque::new();
        Criptomoneda { nombre, prefijo, blockchains }
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_ingresar_dinero(){
        let mut xyz: Plataforma = Plataforma::new("XYZ".to_string());
        let fecha: Fecha = Fecha::new(16, 04, 2002);
        let fiat: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_fiat: (Fiat, f64) = (fiat, 100000.0);
        let mut usuario: Usuario = Usuario::new("Valentino".to_string(),
         "Quiroga".to_string(), "mail.com".to_string(), 44006927, true);
        xyz.registrar_usuario(usuario.clone());
        xyz.ingresar_dinero(fecha, monto_fiat, &mut usuario);
        assert_eq!(xyz.usuarios.len(), 1);
        assert_eq!(xyz.registro.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Usuario no registrado")]
    fn test_ingresar_dinero_usuario_no_registrado(){
        let mut xyz: Plataforma = Plataforma::new("XYZ".to_string());
        let fecha: Fecha = Fecha::new(16, 04, 2002);
        let fiat: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_fiat: (Fiat, f64) = (fiat, 100000.0);
        let mut usuario: Usuario = Usuario::new("Valentino".to_string(),
         "Quiroga".to_string(), "mail.com".to_string(), 44006927, true);
        xyz.ingresar_dinero(fecha, monto_fiat, &mut usuario);
    }

    #[test]
    fn test_comprar_criptomoneda(){
        let mut xyz: Plataforma = Plataforma::new("XYZ".to_string());
        let fecha: Fecha = Fecha::new(16, 04, 2002);
        let fiat: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_fiat: (Fiat, f64) = (fiat.clone(), 100000.0);
        let mut usuario: Usuario = Usuario::new("Valentino".to_string(),
         "Quiroga".to_string(), "mail.com".to_string(), 44006927, true);
        xyz.registrar_usuario(usuario.clone());
        xyz.ingresar_dinero(fecha.clone(), monto_fiat, &mut usuario);

        let mut lista: VecDeque<(String, f64)> = VecDeque::new();
        lista.push_back(("Bitcoin".to_string(), 0.1));
        xyz.tabla_cotizacion.insert("Peso Argentino".to_string(), lista);
        let monto_en_fiat:(Fiat, f64) = (fiat.clone(), 10000.0);
        let cripto: Criptomoneda = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
        usuario.comprar_criptomoneda(fecha.clone(), monto_en_fiat, cripto, &mut xyz);

        assert_eq!(*usuario.balance.get("Peso Argentino").unwrap(), 90000.0);
        assert_eq!(*usuario.balance.get("Bitcoin").unwrap(), 1000.0);
    }

    #[test]
    #[should_panic(expected = "El usuario no cuenta con acceso a la moneda de ingreso")]
    fn test_comprar_criptomoneda_usuario_sin_fiat_ingresado(){
        let mut xyz: Plataforma = Plataforma::new("XYZ".to_string());
        let fecha: Fecha = Fecha::new(16, 04, 2002);
        let fiat: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_fiat: (Fiat, f64) = (fiat.clone(), 100000.0);
        let mut usuario: Usuario = Usuario::new("Valentino".to_string(),
         "Quiroga".to_string(), "mail.com".to_string(), 44006927, true);
        xyz.registrar_usuario(usuario.clone());
        xyz.ingresar_dinero(fecha.clone(), monto_fiat, &mut usuario);

        let mut lista_ars: VecDeque<(String, f64)> = VecDeque::new();
        let mut lista_pen: VecDeque<(String, f64)> = VecDeque::new();
        lista_ars.push_back(("Bitcoin".to_string(), 0.1));
        lista_pen.push_back(("Bitcoin".to_string(), 0.1));
        xyz.tabla_cotizacion.insert("Peso Argentino".to_string(), lista_ars);
        xyz.tabla_cotizacion.insert("Sol Peruano".to_string(), lista_pen);
        
        let fiat_para_cripto: Fiat = Fiat::new("Sol Peruano".to_string(), "PEN".to_string());
        let monto_en_fiat:(Fiat, f64) = (fiat_para_cripto, 10000.0);
        let cripto: Criptomoneda = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());

        usuario.comprar_criptomoneda(fecha.clone(), monto_en_fiat, cripto, &mut xyz);
    }

    #[test]
    #[should_panic(expected = "Esta moneda no cuenta con una cotizacion para la moneda solicitada")]
    fn test_comprar_criptomoneda_usuario_sin_moneda_en_tabla(){
        let mut xyz: Plataforma = Plataforma::new("XYZ".to_string());
        let fecha: Fecha = Fecha::new(16, 04, 2002);
        let fiat: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_fiat: (Fiat, f64) = (fiat.clone(), 100000.0);
        let mut usuario: Usuario = Usuario::new("Valentino".to_string(),
         "Quiroga".to_string(), "mail.com".to_string(), 44006927, true);
        xyz.registrar_usuario(usuario.clone());
        xyz.ingresar_dinero(fecha.clone(), monto_fiat, &mut usuario);

        let mut lista_ars: VecDeque<(String, f64)> = VecDeque::new();
        lista_ars.push_back(("Bitcoin".to_string(), 0.1));
        xyz.tabla_cotizacion.insert("Peso Argentino".to_string(), lista_ars);
        
        let fiat_para_cripto: Fiat = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        let monto_en_fiat:(Fiat, f64) = (fiat_para_cripto, 10000.0);
        let cripto: Criptomoneda = Criptomoneda::new("Sol Peruano".to_string(), "PEN".to_string());

        usuario.comprar_criptomoneda(fecha.clone(), monto_en_fiat, cripto, &mut xyz);
    }
}