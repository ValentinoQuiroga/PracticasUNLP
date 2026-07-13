use std::{collections::{HashMap, VecDeque}, io::BufReader};
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::tp03::ej3::Fecha;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plataforma {
    pub nombre: String,
    pub usuarios: HashMap<u64, Usuario>,
    pub registro: HashMap<u64, VecDeque<Transaccion>>,
    pub tabla_cotizacion: HashMap<String, VecDeque<(String, f64)>>,
    pub path_archivo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub dni: u64,
    pub identidad_validada: bool,
    pub balance: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaccion {
    pub fecha: Fecha,
    pub tipo: String,
    pub monto: f64,
    pub usuario: Usuario,
    pub criptomoneda: Option<String>,
    pub cotizacion: Option<f64>,
    pub blockchain: Option<String>,
    pub hash: Option<String>,
    pub medio_retiro: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fiat {
    pub nombre: String,
    pub prefijo: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criptomoneda {
    pub nombre: String,
    pub prefijo: String,
    pub blockchains: VecDeque<Blockchain>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String,
}

impl Plataforma {
    pub fn new(nombre: String, path_archivo: String) -> Plataforma {
        let mut plataf = Plataforma {
            nombre,
            usuarios: HashMap::new(),
            registro: HashMap::new(),
            tabla_cotizacion: HashMap::new(),
            path_archivo,
        };
        let _ = plataf.guardar_datos();
        plataf
    }

    pub fn guardar_datos(&self) -> Result<(), std::io::Error> {
        let datos_serializados = serde_json::to_string_pretty(self)?;
        let mut archivo = File::create(&self.path_archivo)?;
        archivo.write_all(datos_serializados.as_bytes())?;
        Ok(())
    }

    pub fn cargar_datos(&self) -> Result<Plataforma, std::io::Error> {
        let archivo = File::open(&self.path_archivo)?;
        let lector = BufReader::new(archivo);
        let plataforma_cargada = serde_json::from_reader(lector)?;
        Ok(plataforma_cargada)
    }

    pub fn registrar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.insert(usuario.dni, usuario);
        let _ = self.guardar_datos();
    }

    pub fn registrar_transaccion(&mut self, transaccion: Transaccion) {
        self.registro.entry(transaccion.usuario.dni).or_insert(VecDeque::new()).push_back(transaccion);
        let _ = self.guardar_datos();
    }

    pub fn ingresar_dinero(&mut self, fecha: Fecha, monto_fiat: (Fiat, f64), usuario: &mut Usuario) {
        let tipo: String = "(ingreso de dinero)".to_string();
        let fiat: String = monto_fiat.0.nombre;
        if self.usuarios.contains_key(&usuario.dni) {
            *usuario.balance.entry(fiat.clone()).or_insert(0.0) += monto_fiat.1;
            self.usuarios.insert(usuario.dni, usuario.clone());
            
            let transaccion = Transaccion {
                fecha,
                tipo,
                monto: monto_fiat.1,
                usuario: usuario.clone(),
                criptomoneda: None,
                cotizacion: None,
                blockchain: None,
                hash: None,
                medio_retiro: None,
            };
            self.registrar_transaccion(transaccion);
        } else {
            panic!("Usuario no registrado");
        }
    }

    pub fn obtener_cotizacion(&self, nombre_a: &String, nombre_b: &String) -> f64 {
        if let Some(lista) = self.tabla_cotizacion.get(nombre_a) {
            for m in lista {
                if m.0.eq(nombre_b) {
                    return m.1;
                }
            }
        }
        panic!("Esta moneda no cuenta con una cotizacion para la moneda solicitada");
    }
    
    pub fn cripto_mas_cantidad_ventas(&self) -> Option<String> {
        let mut conteo = HashMap::new();
        for lista in self.registro.values() {
            for tx in lista {
                if tx.tipo == "venta de cripto" {
                    if let Some(ref cripto) = tx.criptomoneda {
                        *conteo.entry(cripto.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(name, _)| name)
    }

    pub fn cripto_mas_cantidad_compras(&self) -> Option<String> {
        let mut conteo = HashMap::new();
        for lista in self.registro.values() {
            for tx in lista {
                if tx.tipo == "compra de cripto" {
                    if let Some(ref cripto) = tx.criptomoneda {
                        *conteo.entry(cripto.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(name, _)| name)
    }

    pub fn cripto_mas_volumen_ventas(&self) -> Option<String> {
        let mut volumenes = HashMap::new();
        for lista in self.registro.values() {
            for tx in lista {
                if tx.tipo == "venta de cripto" {
                    if let Some(ref cripto) = tx.criptomoneda {
                        *volumenes.entry(cripto.clone()).or_insert(0.0) += tx.monto;
                    }
                }
            }
        }
        volumenes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(name, _)| name)
    }

    pub fn cripto_mas_volumen_compras(&self) -> Option<String> {
        let mut volumenes = HashMap::new();
        for lista in self.registro.values() {
            for tx in lista {
                if tx.tipo == "compra de cripto" {
                    if let Some(ref cripto) = tx.criptomoneda {
                        *volumenes.entry(cripto.clone()).or_insert(0.0) += tx.monto;
                    }
                }
            }
        }
        volumenes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(name, _)| name)
    }
}

impl Usuario {
    pub fn new(nombre: String, apellido: String, email: String, dni: u64, identidad_validada: bool) -> Usuario {
        Usuario { nombre, apellido, email, dni, identidad_validada, balance: HashMap::new() }
    }

    pub fn comprar_criptomoneda(&mut self, fecha: Fecha, monto_en_fiat: (Fiat, f64), cripto: Criptomoneda, sistema: &mut Plataforma) {
        if !self.identidad_validada { panic!("Usuario no validado"); }
        let fiat = monto_en_fiat.0.nombre;
        let monto = monto_en_fiat.1;
        
        if let Some(cantidad) = self.balance.get(&fiat) {
            if *cantidad >= monto {
                let cotizacion = sistema.obtener_cotizacion(&fiat, &cripto.nombre);
                let monto_final = monto * cotizacion;
                *self.balance.get_mut(&fiat).unwrap() -= monto;
                *self.balance.entry(cripto.nombre.clone()).or_insert(0.0) += monto_final;
                
                sistema.usuarios.insert(self.dni, self.clone());
                
                let tx = Transaccion {
                    fecha,
                    tipo: "compra de cripto".to_string(),
                    monto: monto_final,
                    usuario: self.clone(),
                    criptomoneda: Some(cripto.nombre),
                    cotizacion: Some(cotizacion),
                    blockchain: None,
                    hash: None,
                    medio_retiro: None,
                };
                sistema.registrar_transaccion(tx);
            } else { panic!("Balance fiat insuficiente"); }
        } else { panic!("El usuario no cuenta con acceso a la moneda de ingreso"); }
    }

    pub fn vender_criptomoneda(&mut self, fecha: Fecha, monto_en_cripto: (Criptomoneda, f64), fiat: Fiat, sistema: &mut Plataforma) {
        if !self.identidad_validada { panic!("Usuario no validado"); }
        let cripto = monto_en_cripto.0.nombre;
        let monto = monto_en_cripto.1;
        
        if let Some(cantidad) = self.balance.get(&cripto) {
            if *cantidad >= monto {
                let cotizacion = sistema.obtener_cotizacion(&cripto, &fiat.nombre);
                let monto_final = monto * cotizacion;
                *self.balance.get_mut(&cripto).unwrap() -= monto;
                *self.balance.entry(fiat.nombre).or_insert(0.0) += monto_final;
                
                sistema.usuarios.insert(self.dni, self.clone());
                
                let tx = Transaccion {
                    fecha,
                    tipo: "venta de cripto".to_string(),
                    monto: monto,
                    usuario: self.clone(),
                    criptomoneda: Some(cripto),
                    cotizacion: Some(cotizacion),
                    blockchain: None,
                    hash: None,
                    medio_retiro: None,
                };
                sistema.registrar_transaccion(tx);
            } else { panic!("Balance cripto insuficiente"); }
        } else { panic!("El usuario no cuenta con acceso a la moneda de ingreso"); }
    }

    pub fn retirar_cripto_a_blockchain(&mut self, fecha: Fecha, monto_en_cripto: (Criptomoneda, f64), mut blockchain: Blockchain, sistema: &mut Plataforma) {
        if !self.identidad_validada { panic!("Usuario no validado"); }
        let cripto_nombre = monto_en_cripto.0.nombre.clone();
        let monto = monto_en_cripto.1.clone();

        if let Some(balance_actual) = self.balance.get_mut(&cripto_nombre) {
            if *balance_actual >= monto {
                *balance_actual -= monto;
                
                let mut cripto_obj = monto_en_cripto.0.clone();
                let hash = cripto_obj.retirar_a_blockchain(monto, blockchain.clone());
                let cotizacion = sistema.obtener_cotizacion(&cripto_nombre, &"Peso Argentino".to_string());

                sistema.usuarios.insert(self.dni, self.clone());

                let tx = Transaccion {
                    fecha,
                    tipo: "retiro cripto".to_string(),
                    monto,
                    usuario: self.clone(),
                    criptomoneda: Some(cripto_nombre),
                    cotizacion: Some(cotizacion),
                    blockchain: Some(blockchain.nombre),
                    hash: Some(hash),
                    medio_retiro: None,
                };
                sistema.registrar_transaccion(tx);
            } else { panic!("Balance cripto insuficiente"); }
        } else { panic!("No posee esa criptomoneda en su balance"); }
    }

    pub fn recibir_criptoneda_de_blockchain(&mut self, fecha: Fecha, monto_en_cripto: (Criptomoneda, f64), blockchain: Blockchain, sistema: &mut Plataforma) {
        let cripto_nombre = monto_en_cripto.0.nombre;
        let monto = monto_en_cripto.1;

        *self.balance.entry(cripto_nombre.clone()).or_insert(0.0) += monto;
        let cotizacion = sistema.obtener_cotizacion(&cripto_nombre, &"Peso Argentino".to_string());

        sistema.usuarios.insert(self.dni, self.clone());

        let tx = Transaccion {
            fecha,
            tipo: "recepción cripto".to_string(),
            monto,
            usuario: self.clone(),
            criptomoneda: Some(cripto_nombre),
            cotizacion: Some(cotizacion),
            blockchain: Some(blockchain.nombre),
            hash: None,
            medio_retiro: None,
        };
        sistema.registrar_transaccion(tx);
    }

    pub fn retirar_fiat(&mut self, fecha: Fecha, monto: f64, medio: String, sistema: &mut Plataforma) {
        if !self.identidad_validada { panic!("Usuario no validado"); }
        if medio != "MercadoPago" && medio != "Transferencia Bancaria" { panic!("Medio inválido"); }

        let fiat_nombre = "Peso Argentino".to_string();
        if let Some(balance_actual) = self.balance.get_mut(&fiat_nombre) {
            if *balance_actual >= monto {
                *balance_actual -= monto;
                sistema.usuarios.insert(self.dni, self.clone());

                let tx = Transaccion {
                    fecha,
                    tipo: "retiro fiat".to_string(),
                    monto,
                    usuario: self.clone(),
                    criptomoneda: None,
                    cotizacion: None,
                    blockchain: None,
                    hash: None,
                    medio_retiro: Some(medio),
                };
                sistema.registrar_transaccion(tx);
            } else { panic!("Balance fiat insuficiente"); }
        } else { panic!("No posee balance en dinero fiat"); }
    }
}

impl Transaccion {
    pub fn new(fecha: Fecha, tipo: String, monto: f64, usuario: Usuario) -> Transaccion {
        Transaccion { fecha, tipo, monto, usuario, criptomoneda: None, cotizacion: None, blockchain: None, hash: None, medio_retiro: None }
    }
}

impl Fiat {
    pub fn new(nombre: String, prefijo: String) -> Fiat {
        Fiat { nombre, prefijo }
    }
}

impl Criptomoneda {
    pub fn new(nombre: String, prefijo: String) -> Criptomoneda {
        Criptomoneda { nombre, prefijo, blockchains: VecDeque::new() }
    }
    pub fn retirar_a_blockchain(&mut self, _monto: f64, blockchain: Blockchain) -> String {
        self.blockchains.push_back(blockchain.clone());
        let mut rng: u32 = rand::random();
        format!("{}{}", blockchain.nombre, rng)
    }
}

impl Blockchain {
    pub fn new(nombre: String, prefijo: String) -> Blockchain {
        Blockchain { nombre, prefijo }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn obtener_archivo_temporal(nombre_test: &str) -> String {
        format!("src/test_plataforma_{}.json", nombre_test)
    }/* 
    #[test]
    fn test_flujo_principal_y_estadisticas() {
        let ruta_archivo = obtener_archivo_temporal("principal");
        let mut plataforma_xyz = Plataforma::new("XYZ".to_string(), ruta_archivo.clone());
        
        let mut usuario_valido = Usuario::new(
            "Valentino".to_string(), 
            "Quiroga".to_string(), 
            "valen@mail.com".to_string(), 
            44006927, 
            true
        );
        plataforma_xyz.registrar_usuario(usuario_valido.clone());

        let pesos = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());
        plataforma_xyz.ingresar_dinero(Fecha::new(16, 4, 2026), (pesos.clone(), 150000.0), &mut usuario_valido);

        let mut lista_cotizaciones_pesos = VecDeque::new();
        lista_cotizaciones_pesos.push_back(("Bitcoin".to_string(), 0.1));
        lista_cotizaciones_pesos.push_back(("Ethereum".to_string(), 0.05));
        plataforma_xyz.tabla_cotizacion.insert("Peso Argentino".to_string(), lista_cotizaciones_pesos);

        let mut lista_cotizaciones_btc = VecDeque::new();
        lista_cotizaciones_btc.push_back(("Peso Argentino".to_string(), 10.0));
        plataforma_xyz.tabla_cotizacion.insert("Bitcoin".to_string(), lista_cotizaciones_btc);

        let bitcoin = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
        let ethereum = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());

        usuario_valido.comprar_criptomoneda(Fecha::new(17, 4, 2026), (pesos.clone(), 60000.0), bitcoin.clone(), &mut plataforma_xyz);
        usuario_valido.comprar_criptomoneda(Fecha::new(18, 4, 2026), (pesos.clone(), 20000.0), ethereum.clone(), &mut plataforma_xyz);
        
        usuario_valido.vender_criptomoneda(Fecha::new(19, 4, 2026), (bitcoin.clone(), 1000.0), pesos.clone(), &mut plataforma_xyz);

        assert_eq!(plataforma_xyz.cripto_mas_cantidad_compras(), Some("Bitcoin".to_string()));
        assert_eq!(plataforma_xyz.cripto_mas_cantidad_ventas(), Some("Bitcoin".to_string()));
        assert_eq!(plataforma_xyz.cripto_mas_volumen_compras(), Some("Bitcoin".to_string()));
        assert_eq!(plataforma_xyz.cripto_mas_volumen_ventas(), Some("Bitcoin".to_string()));

        let plataforma_recuperada = plataforma_xyz.cargar_datos().unwrap();
        assert_eq!(plataforma_recuperada.nombre, "XYZ");
        assert!(plataforma_recuperada.usuarios.contains_key(&44006927));

        let _ = fs::remove_file(ruta_archivo);
    }*/

    #[test]
    #[should_panic(expected = "Usuario no validado")]
    fn test_usuario_no_validado_panico() {
        let ruta_archivo = obtener_archivo_temporal("no_validado");
        let mut plataforma_xyz = Plataforma::new("XYZ".to_string(), ruta_archivo.clone());
        
        let mut usuario_invalido = Usuario::new(
            "Carlos".to_string(), 
            "Gomez".to_string(), 
            "carlos@mail.com".to_string(), 
            12345678, 
            false
        );
        let bitcoin = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
        let pesos = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());

        usuario_invalido.comprar_criptomoneda(Fecha::new(16, 4, 2026), (pesos, 1000.0), bitcoin, &mut plataforma_xyz);
        let _ = fs::remove_file(ruta_archivo);
    }

    #[test]
    #[should_panic(expected = "Balance fiat insuficiente")]
    fn test_balance_insuficiente_panico() {
        let ruta_archivo = obtener_archivo_temporal("balance_insuficiente");
        let mut plataforma_xyz = Plataforma::new("XYZ".to_string(), ruta_archivo.clone());
        
        let mut usuario_pobre = Usuario::new(
            "Ana".to_string(), 
            "Lopez".to_string(), 
            "ana@mail.com".to_string(), 
            87654321, 
            true
        );
        usuario_pobre.balance.insert("Peso Argentino".to_string(), 500.0);
        
        let bitcoin = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
        let pesos = Fiat::new("Peso Argentino".to_string(), "ARS".to_string());

        usuario_pobre.comprar_criptomoneda(Fecha::new(16, 4, 2026), (pesos, 5000.0), bitcoin, &mut plataforma_xyz);
        let _ = fs::remove_file(ruta_archivo);
    }

    #[test]
    fn test_movimientos_blockchain() {
        let ruta_archivo = obtener_archivo_temporal("blockchain");
        let mut plataforma_xyz = Plataforma::new("XYZ".to_string(), ruta_archivo.clone());
        
        let mut usuario_valido = Usuario::new(
            "Luis".to_string(), 
            "Perez".to_string(), 
            "luis@mail.com".to_string(), 
            11223344, 
            true
        );

        let mut lista_cotizaciones = VecDeque::new();
        lista_cotizaciones.push_back(("Peso Argentino".to_string(), 1.0));
        plataforma_xyz.tabla_cotizacion.insert("Bitcoin".to_string(), lista_cotizaciones);

        let bitcoin = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
        let red_blockchain = Blockchain::new("Lightning Network".to_string(), "LN".to_string());

        usuario_valido.recibir_criptoneda_de_blockchain(Fecha::new(16, 4, 2026), (bitcoin.clone(), 5.0), red_blockchain.clone(), &mut plataforma_xyz);
        assert_eq!(*usuario_valido.balance.get("Bitcoin").unwrap(), 5.0);

        usuario_valido.retirar_cripto_a_blockchain(Fecha::new(17, 4, 2026), (bitcoin.clone(), 2.0), red_blockchain.clone(), &mut plataforma_xyz);
        assert_eq!(*usuario_valido.balance.get("Bitcoin").unwrap(), 3.0);
        
        let _ = fs::remove_file(ruta_archivo);
    }

    #[test]
    fn test_retirar_fiat() {
        let ruta_archivo = obtener_archivo_temporal("retiro_fiat");
        let mut plataforma_xyz = Plataforma::new("XYZ".to_string(), ruta_archivo.clone());
        
        let mut usuario_valido = Usuario::new(
            "Maria".to_string(), 
            "Rodriguez".to_string(), 
            "maria@mail.com".to_string(), 
            55667788, 
            true
        );
        usuario_valido.balance.insert("Peso Argentino".to_string(), 20000.0);

        usuario_valido.retirar_fiat(Fecha::new(16, 4, 2026), 5000.0, "MercadoPago".to_string(), &mut plataforma_xyz);
        assert_eq!(*usuario_valido.balance.get("Peso Argentino").unwrap(), 15000.0);
        
        let _ = fs::remove_file(ruta_archivo);
    }
}