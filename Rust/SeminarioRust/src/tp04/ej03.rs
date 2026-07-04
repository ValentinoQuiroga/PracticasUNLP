use std::collections::HashMap;

use crate::tp03::ej3::Fecha;

struct PlataformaStreaming{
    nombre: String,
    usuarios: HashMap<u32,Usuario>
}

#[derive(Debug, Clone, PartialEq)]
struct Suscripcion{
    costo_mensual: f64,
    duracion_meses: u16,
    fecha_inicio: Fecha
}

#[derive(Debug, Clone, PartialEq)]
struct Usuario{
    suscripcion: Option<Suscripcion>,
    tipo_suscripcion: TipoSuscripcion,
    metodo: MetodoDePago
}

#[derive(Debug, Clone, PartialEq)]
enum TipoSuscripcion{
    Basic,
    Classic,
    Super
}

#[derive(Debug, Clone, PartialEq)]
enum MetodoDePago{
    Efectivo,
    MercadoPago,
    TarjetaDeCredito,
    TransferenciaBancaria,
    Cripto
}

impl PlataformaStreaming{

    pub fn metodo_mas_utilizado_activa(&self) -> Option<MetodoDePago>{
        let mut iter_s: Vec<Usuario> = 
            self.usuarios.clone().into_values().filter(|u| u.suscripcion.is_some()).collect();

        return self.metodo_mas_utilizado(iter_s);
    }
    pub fn metodo_mas_utilizado_general(&self) -> Option<MetodoDePago>{
        let mut iter_s: Vec<Usuario> = self.usuarios.clone().into_values().collect();
        return self.metodo_mas_utilizado(iter_s);
    }

    pub fn metodo_mas_utilizado(&self, iter_s: Vec<Usuario>) -> Option<MetodoDePago>{
        let mut max: (MetodoDePago, u32) = (MetodoDePago::Efectivo, 0);
        let mut contador = [(MetodoDePago::Cripto, 0), (MetodoDePago::Efectivo, 0), 
                            (MetodoDePago::MercadoPago, 0), (MetodoDePago::TarjetaDeCredito, 0), 
                            (MetodoDePago::TransferenciaBancaria, 0)];

        for u in iter_s{
            match u.metodo{
                MetodoDePago::Cripto => {contador[0].1 += 1},
                MetodoDePago::Efectivo => {contador[1].1 += 1},
                MetodoDePago::MercadoPago => {contador[2].1 += 1},
                MetodoDePago::TarjetaDeCredito => {contador[3].1 += 1},
                MetodoDePago::TransferenciaBancaria => {contador[4].1 += 1}
            }
        }

        for c in contador{
            if c.1 > max.1{
                max = c
            }
        }
        if max.1 == 0{ return None}
        else{ return Some(max.0)}
    }
    pub fn suscripcion_mas_contratada_activa(&self) -> Option<TipoSuscripcion>{
        let mut iter_s: Vec<Usuario> = 
            self.usuarios.clone().into_values().filter(|u| u.suscripcion.is_some()).collect();

        return self.suscripcion_mas_contratada(iter_s);
    }
    pub fn suscripcion_mas_contratada_general(&self) -> Option<TipoSuscripcion>{
        let mut iter_s: Vec<Usuario> = self.usuarios.clone().into_values().collect();
        return self.suscripcion_mas_contratada(iter_s);
    }
    fn suscripcion_mas_contratada(&self, iter_s: Vec<Usuario>) -> Option<TipoSuscripcion>{
        let mut max: (TipoSuscripcion, u32) = (TipoSuscripcion::Basic, 0);
        let s = Suscripcion::default();
        let mut contador = [(TipoSuscripcion::Basic, 0),
                        (TipoSuscripcion::Classic, 0), (TipoSuscripcion::Super, 0) ];

        for u in iter_s{
            match u.tipo_suscripcion{
                TipoSuscripcion::Basic => {contador[0].1 += 1},
                TipoSuscripcion::Classic => {contador[1].1 += 1},
                TipoSuscripcion::Super => {contador[2].1 += 1}
            }
        }

        for c in contador{
            if c.1 > max.1{
                max = c
            }
        }
        if max.1 == 0{ return None}
        else{ return Some(max.0)}
    }

    pub fn agregar_usuario(&mut self, usuario: Usuario){
        self.usuarios.insert(self.usuarios.len() as u32, usuario);
    }
}

impl Suscripcion{
    pub fn default() -> Suscripcion{
        Suscripcion { costo_mensual: 10000.0, duracion_meses: 12, fecha_inicio: Fecha::new(16,04,2002) }
    }
}
impl TipoSuscripcion{
    pub fn upgrade(usuario: &mut Usuario){
        match(& mut usuario.tipo_suscripcion){
            TipoSuscripcion::Basic => (usuario.tipo_suscripcion = TipoSuscripcion::Classic),
            TipoSuscripcion::Classic => (usuario.tipo_suscripcion = TipoSuscripcion::Super),
            _ => ()
        }
    }
    pub fn downgrade(usuario: &mut Usuario){
        match(& mut usuario.tipo_suscripcion){
            TipoSuscripcion::Basic => (usuario.suscripcion = None),
            TipoSuscripcion::Classic => (usuario.tipo_suscripcion = TipoSuscripcion::Basic),
            TipoSuscripcion::Super => (usuario.tipo_suscripcion = TipoSuscripcion::Classic),
            _ => ()
        }
    }
    pub fn cancelar(usuario: &mut Usuario){
        usuario.suscripcion = None;
    }
}

impl Usuario{
    pub fn new(suscripcion: Option<Suscripcion>, tipo_suscripcion: TipoSuscripcion, metodo: MetodoDePago) -> Usuario{
        Usuario { suscripcion, tipo_suscripcion, metodo }
    }
    pub fn upgrade_suscripcion(&mut self){
        TipoSuscripcion::upgrade(self);    
    }
    pub fn downgrade_suscripcion(&mut self){
        TipoSuscripcion::downgrade(self);
    }
    pub fn cancelar_suscripcion(&mut self){
        TipoSuscripcion::cancelar(self);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_crear_usuario(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Basic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let u: Usuario = Usuario::new(Some(s), ts, m);

        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Super);
    }
    
    #[test]
    fn test_mejorar_suscripcion_basic(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Basic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.upgrade_suscripcion();
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
    }

    #[test]
    fn test_mejorar_suscripcion_vacia(){
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let ts: TipoSuscripcion = TipoSuscripcion::Basic;
        let mut u: Usuario = Usuario::new(None, ts, m);

        assert_eq!(u.suscripcion.is_none(), true);

        u.upgrade_suscripcion();
        assert_eq!(u.suscripcion.is_none(), true);
    }

    #[test]
    fn test_mejorar_suscripcion_classic(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Classic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.upgrade_suscripcion();
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Super);
    }

    #[test]
    fn test_mejorar_suscripcion_super(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Super;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.upgrade_suscripcion();
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Super);
    }

    #[test]
    fn test_degradar_suscripcion_basic(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Basic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.downgrade_suscripcion();
        assert_eq!(u.suscripcion.is_none(), true);
    }

    #[test]
    fn test_degradar_suscripcion_vacia(){
        let ts: TipoSuscripcion = TipoSuscripcion::Basic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(None, ts, m);

        assert_eq!(u.suscripcion.is_none(), true);

        u.downgrade_suscripcion();
        assert_eq!(u.suscripcion.is_none(), true);
    }

    #[test]
    fn test_degradar_suscripcion_classic(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Classic;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.downgrade_suscripcion();
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
    }

    #[test]
    fn test_degradar_suscripcion_super(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Super;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
        assert_ne!(&u.tipo_suscripcion, &TipoSuscripcion::Basic);
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Super);

        u.downgrade_suscripcion();
        assert_eq!(&u.tipo_suscripcion, &TipoSuscripcion::Classic);
    }

    #[test]
    fn test_cancelar_suscripcion(){
        let s: Suscripcion = Suscripcion::default();
        let ts: TipoSuscripcion = TipoSuscripcion::Super;
        let m: MetodoDePago = MetodoDePago::MercadoPago;
        let mut u: Usuario = Usuario::new(Some(s), ts, m);

        assert_eq!(u.suscripcion.is_some(), true);
        
        u.cancelar_suscripcion();
        assert_eq!(u.suscripcion.is_none(), true);
    }

    #[test]
    fn test_metodo_de_pago_suscripciones_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        //Plantea una mayoria Efectivo, pero una mayoria activa MercadoPago
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });

        assert_eq!(streaming_rust.metodo_mas_utilizado_activa().unwrap(), MetodoDePago::MercadoPago);
    }

    #[test]
    fn test_metodo_de_pago_suscripciones_activas_sin_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::TransferenciaBancaria });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Cripto });

        assert_eq!(streaming_rust.metodo_mas_utilizado_activa().is_none(), true);
    }

    #[test]
    fn test_metodo_de_pago_suscripciones_generales(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        //Plantea una mayoria Efectivo, pero una mayoria activa MercadoPago
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });

        assert_eq!(streaming_rust.metodo_mas_utilizado_general().unwrap(), MetodoDePago::Efectivo);
    }

    #[test]
    fn test_metodo_de_pago_suscripciones_generales_sin_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        assert_eq!(streaming_rust.metodo_mas_utilizado_general().is_none(), true);
    }

    #[test]
    fn test_suscripcion_mas_contratada_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        //Plantea una mayoria Basic, pero una mayoria activa Classic
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Classic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Classic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });

        assert_eq!(streaming_rust.suscripcion_mas_contratada_activa().unwrap(), TipoSuscripcion::Classic);
    }

    #[test]
    fn test_suscripcion_mas_contratada_activas_sin_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::TransferenciaBancaria });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Cripto });

        assert_eq!(streaming_rust.suscripcion_mas_contratada_activa().is_none(), true);
    }

    #[test]
    fn test_suscripcion_mas_contratada_generales(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Classic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: Some(Suscripcion::default()), tipo_suscripcion: TipoSuscripcion::Classic, metodo: MetodoDePago::MercadoPago });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });
        streaming_rust.agregar_usuario(Usuario { suscripcion: None, tipo_suscripcion: TipoSuscripcion::Basic, metodo: MetodoDePago::Efectivo });

        assert_eq!(streaming_rust.suscripcion_mas_contratada_general().unwrap(), TipoSuscripcion::Basic);
    }

    #[test]
    fn test_suscripcion_mas_contratada_generales_sin_activas(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre:String = "StreamingRust".to_string();
        let mut streaming_rust = PlataformaStreaming{nombre, usuarios};

        assert_eq!(streaming_rust.suscripcion_mas_contratada_general().is_none(), true);
    }

}