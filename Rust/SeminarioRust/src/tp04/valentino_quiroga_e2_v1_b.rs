use std::{collections::{HashMap, VecDeque}, hash::Hash};
#[derive(Clone, Debug, PartialEq)]
pub struct Fecha{
    dd: u32,
    mm: u32,
    aaaa: u32,
}

impl Fecha{
    pub fn ig(&self, otra_fecha: &Fecha) -> bool{
        if (self.dd != otra_fecha.dd) || (self.mm != otra_fecha.mm) || (self.aaaa != otra_fecha.aaaa){
            false
        }else{true}
    }
    pub fn new(dd: u32, mm: u32, aaaa: u32) -> Fecha{
        Fecha{dd,mm,aaaa}
    }
    pub fn es_fecha_valida(&self) -> bool{
        let mut dia_max: u32;
        match self.mm{
            1|3|5|7|8|10|12 => (dia_max = 31),
            2 => {
                if self.es_bisiesto(){
                    dia_max = 29;
                }else{
                    dia_max = 28;
                }
            }
            _ => dia_max = 30,
        }

        if self.dd > dia_max{
            return false
        }else{
            return true
        }
    }
    pub fn es_bisiesto(&self) -> bool{
        (self.aaaa % 4 == 0) & ((self.aaaa % 100 != 0) | (self.aaaa % 400 == 0))
    }

    pub fn sumar_dias(&mut self, dias: u32){
        self.dd += dias;
        let mut dia_max: u32;
        let mut aumentar_aaaa = false;
        match self.mm{
            1|3|5|7|8|10 => dia_max = 31,
            12 => {
                dia_max = 31;
                aumentar_aaaa = true;
            },
            2 => {
                if self.es_bisiesto(){
                    dia_max = 29;
                }else{
                    dia_max = 28;
                }
            }
            _ => dia_max = 30,
        }
        if self.dd > dia_max{
            if aumentar_aaaa{
                self.aaaa += 1;
                self.mm = 1
            }else{
                self.mm += 1;
            }
            self.dd -= dia_max;
        }
    }

    pub fn restar_dias(&mut self, dias: i32){
        let mut dia_aux: i32 = self.dd  as i32;
        dia_aux -= dias;
        if dia_aux < 1{
            let mut dia_max_mes_anterior: i32;
            let mut decrementar_aaaa = false;
            match self.mm{
                12|5|7|10 => dia_max_mes_anterior = 30,
                1 => {
                    dia_max_mes_anterior = 31;
                    decrementar_aaaa = true;
                },
                3 => {
                    if self.es_bisiesto(){
                        dia_max_mes_anterior = 29;
                    }else{
                        dia_max_mes_anterior = 28;
                    }
                }
                _ => dia_max_mes_anterior = 31,
            }
            if decrementar_aaaa{
                self.aaaa -= 1;
                self.mm = 12
            }else{
                self.mm -= 1;
            }
            self.dd = (dia_aux + dia_max_mes_anterior) as u32;
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool{
        if (una_fecha.aaaa < self.aaaa){return true}

        else if (una_fecha.mm < self.mm){return true}
    
        else if (una_fecha.dd < self.dd){return true}
        else{ return false}

    }

    pub fn obetener_estacion(&self) -> Estacion{
        match (self.mm, self.dd){
            (12, 21..31) | (1|2, _) | (3, 1..20)=> (Estacion::Verano),

            (3, 21..31) | (4|5, _) | (6, 1..20) => (Estacion::Otoño),

            (6, 21..31) | (7|8, _) | (9, 1..20) => (Estacion::Invierno),

            (9, 21..31) | (10|11, _) | (12, 1..20) => (Estacion::Primavera),

            _ => panic!("Fecha invalida")
        }
    }
}

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

#[derive(PartialEq)]
enum Estacion{
    Verano,
    Otoño,
    Invierno,
    Primavera
}
#[derive(PartialEq)]
struct EstacionTop{
    estacion: Estacion,
    cantidad_estacion: u64,
    mes_top: u8,
    cantidad_mes: u64
}

impl EstacionTop{
    pub fn new(estacion: Estacion, cantidad_estacion: u64, mes_top: u8, cantidad_mes:u64) -> EstacionTop{
        EstacionTop { estacion, cantidad_estacion, mes_top, cantidad_mes }
    }
}
impl PlataformaStreaming{

    pub fn estacion_con_mas_suscripciones(&self) -> Option<EstacionTop>{
        let mut lista: VecDeque<Option<Suscripcion>> = self.usuarios.clone().into_iter().map(|x| x.1.suscripcion).filter(|x| x.is_some()).collect();

        if lista.is_empty(){
            return None
        }

        let mut data_estaciones: [[u64; 13]; 4] = [[0; 13]; 4];
        for susc in lista{
            let s = susc.unwrap();
                let mes = s.fecha_inicio.mm;
            match s.fecha_inicio.obetener_estacion(){
                (Estacion::Verano) => {
                    data_estaciones[0][mes as usize] += 1;
                },
                (Estacion::Otoño) => {
                    data_estaciones[1][mes as usize] += 1;
                },
                (Estacion::Invierno) => {
                    data_estaciones[2][mes as usize] += 1;
                },
                (Estacion::Primavera) => {
                    data_estaciones[3][mes as usize] += 1;
                },            
            }
        }

        let mut est_max: (u8, u64) = (0, 0);
        let mut mes_est_max:(u8, u64) = (0, 0);
        for i_est in 0..4{
            let mut est_act:(u8, u64) = (i_est, 0);
            let mut mes_max:(u8, u64) = (0, 0);
            for i_mes in 1..13{
                let mes_act:(u8, u64) = (i_mes, data_estaciones[i_est as usize][i_mes as usize]);
                est_act.1 += mes_act.1;
                if mes_act.1 > mes_max.1{
                    mes_max = mes_act;
                }
            }
            if est_act.1 > est_max.1{
                est_max = est_act;
                mes_est_max = mes_max;
            }
        }

        let mut estacion: Estacion;
        match est_max.0{
            0 => estacion = Estacion::Verano,
            1 => estacion = Estacion::Otoño,
            2 => estacion = Estacion::Invierno,
            3 => estacion = Estacion::Primavera,
            _ => panic!("Se proceso una estacion invalida")
        }
        return Some(EstacionTop::new(estacion, est_max.1, mes_est_max.0, mes_est_max.1))
    }


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
    fn test_estacion_top_cero_usuarios(){
        let usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_none(), true);
    }

    #[test]
    fn test_estacion_top_cero_suscripciones(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let usuario = Usuario::new(None, TipoSuscripcion::Basic, MetodoDePago::Efectivo);
        for i in 0..4{
            usuarios.insert(i, usuario.clone());
        }
        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_none(), true);
    }

    #[test]
    fn test_estacion_top_ganador_otoño(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let sus_otoño = Suscripcion::default();
        let mut sus_verano = Suscripcion::default();
        let fecha_verano = Fecha::new(8, 01, 2002);
        sus_verano.fecha_inicio = fecha_verano; //Estas atrocidades pasan por olvidarme los new()

        let us_otoño = Usuario::new(Some(sus_otoño), TipoSuscripcion::Basic, MetodoDePago::Efectivo);
        let us_verano = Usuario::new(Some(sus_verano), TipoSuscripcion::Basic, MetodoDePago::Efectivo);

        for i in 0..4{
            usuarios.insert(i, us_otoño.clone());
        }
        usuarios.insert(4, us_verano);

        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        let estacion_top_esperada = EstacionTop::new(Estacion::Otoño,
                         4, 4, 4);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_some(), true);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().unwrap().eq(&estacion_top_esperada), true);
    }

    #[test]
    fn test_estacion_top_empate_primavera_invierno(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let mut sus_primavera = Suscripcion::default();
        let fecha_primavera = Fecha::new(8, 10, 2002);
        sus_primavera.fecha_inicio = fecha_primavera;
        let mut sus_invierno = Suscripcion::default();
        let fecha_invierno = Fecha::new(31, 8, 2005);
        sus_invierno.fecha_inicio = fecha_invierno; //Estas atrocidades pasan por olvidarme los new()

        let us_primavera = Usuario::new(Some(sus_primavera), TipoSuscripcion::Basic, MetodoDePago::Efectivo);
        let us_invierno = Usuario::new(Some(sus_invierno), TipoSuscripcion::Basic, MetodoDePago::Efectivo);

        for i in 0..4{
            usuarios.insert(i, us_primavera.clone());
        }
        for i in 4..8{
            usuarios.insert(i, us_invierno.clone());
        }

        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        let estacion_top_esperada = EstacionTop::new(Estacion::Invierno,
                         4, 8, 4);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_some(), true);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().unwrap().eq(&estacion_top_esperada), true);
    }

    #[test]
    fn test_estacion_top_ganador_verano(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let mut sus_verano = Suscripcion::default();
        let fecha_verano = Fecha::new(8, 1, 2002);
        sus_verano.fecha_inicio = fecha_verano.clone();
        fecha_verano.ig(&sus_verano.fecha_inicio);

        let us_verano = Usuario::new(Some(sus_verano), TipoSuscripcion::Basic, MetodoDePago::Efectivo);
        for i in 0..4{
            usuarios.insert(i, us_verano.clone());
        }

        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        let estacion_top_esperada = EstacionTop::new(Estacion::Verano,
                         4, 1, 4);
                         
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_some(), true);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().unwrap().eq(&estacion_top_esperada), true);
    }

    #[test]
    fn test_estacion_top_ganador_primavera(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let mut sus_primavera = Suscripcion::default();
        let fecha_primavera = Fecha::new(8, 10, 2002);
        sus_primavera.fecha_inicio = fecha_primavera;

        let us_primavera = Usuario::new(Some(sus_primavera), TipoSuscripcion::Basic, MetodoDePago::Efectivo);
        for i in 0..4{
            usuarios.insert(i, us_primavera.clone());
        }

        let nombre = "a".to_string();
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        let estacion_top_esperada = EstacionTop::new(Estacion::Primavera,
                         4, 10, 4);
                         
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().is_some(), true);
        assert_eq!(streaming_rust.estacion_con_mas_suscripciones().unwrap().eq(&estacion_top_esperada), true);
    }

    #[test]
    #[should_panic(expected = "Fecha invalida")]
    fn test_estacion_top_ingreso_fecha_invalida(){
        let mut usuarios: HashMap<u32, Usuario> = HashMap::new();
        let nombre = "a".to_string();
        
        let mut sus_verano = Suscripcion::default();
        let fecha_verano = Fecha::new(99, 99, 2002);
        sus_verano.fecha_inicio = fecha_verano; //Estas atrocidades pasan por olvidarme los new()

        let us_verano = Usuario::new(Some(sus_verano), TipoSuscripcion::Basic, MetodoDePago::Efectivo);

        usuarios.insert(0, us_verano);
        let streaming_rust = PlataformaStreaming{nombre, usuarios};
        streaming_rust.estacion_con_mas_suscripciones();
    }
    
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

    #[test]
    fn test_crear_fecha(){
        let fecha = Fecha::new(16, 04, 2002);
    }

    #[test]
    fn test_crear_fecha_dia_invalido(){
        let fecha = Fecha::new(32, 10, 2002);
        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn test_crear_fecha_mes_invalido(){
        let fecha = Fecha::new(31, 13, 2002);
        assert_eq!(fecha.es_fecha_valida(), false)
    }

    #[test]
    fn test_crear_fecha_bisiesto(){
        let fecha = Fecha::new(31, 12, 2024);
        assert_eq!(fecha.es_bisiesto(), true)
    }

    #[test]
    fn test_crear_fecha_no_bisiesto(){
        let fecha = Fecha::new(31, 12, 2026);
        assert_eq!(fecha.es_bisiesto(), false)
    }

    #[test]
    fn test_sumar_fecha_no_bisiesto(){
        let mut fecha = Fecha::new(31, 12, 2026);
        fecha.sumar_dias(7);
        assert_eq!(fecha.dd, 7);
        assert_eq!(fecha.mm, 1);
        assert_eq!(fecha.aaaa, 2027);
    }

    #[test]
    fn test_sumar_fecha_bisiesto(){
        let mut fecha = Fecha::new(28, 2, 2024);
        fecha.sumar_dias(7);
        assert_eq!(fecha.dd, 6);
        assert_eq!(fecha.mm, 3);
        assert_eq!(fecha.aaaa, 2024);
    }

    #[test]
    fn test_restar_fecha_no_bisiesto(){
        let mut fecha = Fecha::new(1, 1, 2026);
        fecha.restar_dias(31);
        assert_eq!(fecha.dd, 1);
        assert_eq!(fecha.mm, 12);
        assert_eq!(fecha.aaaa, 2025);
    }

    #[test]
    fn test_restar_fecha_bisiesto(){
        let mut fecha = Fecha::new(3, 3, 2024);
        fecha.restar_dias(4);
        assert_eq!(fecha.dd, 28);
        assert_eq!(fecha.mm, 2);
        assert_eq!(fecha.aaaa, 2024);
    }

    #[test]
    fn test_es_fecha_mayor(){
        let mut fecha_a = Fecha::new(2, 1, 2026);
        let mut fecha_b = Fecha::new(1, 1, 2026);
        assert_eq!(fecha_a.es_mayor(&fecha_b), true);
    }

    #[test]
    fn test_no_es_fecha_mayor(){
        let mut fecha_a = Fecha::new(1, 1, 2026);
        let mut fecha_b = Fecha::new(1, 1, 2026);
        assert_eq!(fecha_b.es_mayor(&fecha_a), false);
    }

    #[test]
    fn test_es_fecha_igual(){
        let mut fecha_a = Fecha::new(1, 1, 2026);
        let mut fecha_b = Fecha::new(1, 1, 2026);
        assert_eq!(fecha_b.es_mayor(&fecha_a), false);
        assert_eq!(fecha_a.es_mayor(&fecha_b), false);
    }

}
