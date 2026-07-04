use std::collections::{HashMap, VecDeque};

use crate::tp03::ej3::Fecha;

struct Plataforma{
    nombre: String,
    usuarios: HashMap<u64,Usuario>,
    registro: HashMap<u64, VecDeque<Transaccion>>
}

struct Usuario{
    nombre: String,
    apellido: String,
    email: String,
    dni: u64,
    identidad_validada: bool,
    balance: HashMap<String, f64>,
}
struct Transaccion{
    fecha: Fecha,
    tipo: String,
    monto: f64,
    usuario: &Usuario
}
struct Fiat{
    nombre: String,
    prefijo: String
}
struct Criptomoneda{
    nombre: String,
    prefijo: String,
    blockchains: VecDeque<Blockchain>
}

struct Blockchain{
    nombre: String,
    prefijo: String
}

impl Plataforma{
    pub fn registrar_usuario(&mut self, usuario: Usuario){
        self.usuarios.insert(usuario.dni, usuario);
    }
    pub fn ingresar_dinero(&mut self, fecha: Fecha, monto_fiat: (Fiat, f64), usuario: &mut Usuario){
        let tipo: String = monto_fiat.0.nombre.clone();
        *usuario.balance.entry(tipo.clone()).or_insert(0.0) += monto_fiat.1;
        self.registrar_transaccion(fecha, tipo, monto_fiat.1, &usuario);
    }
    pub fn registrar_transaccion(&mut self, fecha: Fecha, tipo: String, monto: f64, usuario: &Usuario){
        let transaccion: Transaccion = Transaccion::new(fecha, tipo, monto, &usuario);
        self.registro.entry(usuario.dni).or_insert(VecDeque::new()).push_back(transaccion);
    }
}
impl Usuario{
    pub fn new(nombre: String, apellido: String, email: String, 
        dni: u64, identidad_validada: bool) -> Usuario{
        let balance: HashMap<String, f64> = HashMap::new();
        Usuario { nombre, apellido, email, dni, identidad_validada, balance }
    }
}
impl Transaccion{
    pub fn new(fecha: Fecha, tipo: String, monto: f64, usuario: &Usuario) -> Transaccion{
        Transaccion { fecha, tipo, monto, usuario }
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_
}