pub struct Fecha{
    dd: u32,
    mm: u32,
    aaaa: u32,
}

impl Fecha{
    pub fn new(dd: u32, mm: u32, aaaa: u32) -> Fecha{
        Fecha{dd,mm,aaaa}
    }
    pub fn es_fecha_valida(&self) -> bool{
        match self.dd{
            1..32 => (),
            _ => return false
        }

        match self.mm{
            1..13 => return true,
            _ => return false
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn crearFecha(){
        let fecha = Fecha::new(16, 04, 2002);
    }

    #[test]
    fn crearFechaDiaInvalido(){
        let fecha = Fecha::new(32, 10, 2002);
        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn crearFechaMesInvalido(){
        let fecha = Fecha::new(31, 13, 2002);
        assert_eq!(fecha.es_fecha_valida(), false)
    }

}