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
        let mut diaMax: u32;
        match self.mm{
            1|3|5|7|8|10|12 => (diaMax = 31),
            2 => {
                if self.es_bisiesto(){
                    diaMax = 29;
                }else{
                    diaMax = 28;
                }
            }
            _ => diaMax = 30,
        }

        if self.dd > diaMax{
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
        let mut diaMax: u32;
        let mut aumentarAaaa = false;
        match self.mm{
            1|3|5|7|8|10 => diaMax = 31,
            12 => {
                diaMax = 31;
                aumentarAaaa = true;
            },
            2 => {
                if self.es_bisiesto(){
                    diaMax = 29;
                }else{
                    diaMax = 28;
                }
            }
            _ => diaMax = 30,
        }
        if self.dd > diaMax{
            if aumentarAaaa{
                self.aaaa += 1;
                self.mm = 1
            }else{
                self.mm += 1;
            }
            self.dd -= diaMax;
        }
    }

    pub fn restar_dias(&mut self, dias: i32){
        let mut diaAux: i32 = self.dd  as i32;
        diaAux -= dias;
        if diaAux < 1{
            let mut diaMaxMesAnterior: i32;
            let mut decrementarAaaa = false;
            match self.mm{
                12|5|7|10 => diaMaxMesAnterior = 30,
                1 => {
                    diaMaxMesAnterior = 31;
                    decrementarAaaa = true;
                },
                3 => {
                    if self.es_bisiesto(){
                        diaMaxMesAnterior = 29;
                    }else{
                        diaMaxMesAnterior = 28;
                    }
                }
                _ => diaMaxMesAnterior = 31,
            }
            if decrementarAaaa{
                self.aaaa -= 1;
                self.mm = 12
            }else{
                self.mm -= 1;
            }
            self.dd = (diaAux + diaMaxMesAnterior) as u32;
        }
    }

    pub fn es_mayor(&self, una_fecha: Fecha) -> bool{
        if (una_fecha.aaaa > self.aaaa){return true}

        else if (una_fecha.mm > self.mm){return true}
    
        else if (una_fecha.dd > self.dd){return true}
        else{ return false}

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

    #[test]
    fn crearFechaBisiesto(){
        let fecha = Fecha::new(31, 12, 2024);
        assert_eq!(fecha.es_bisiesto(), true)
    }

    #[test]
    fn crearFechaNoBisiesto(){
        let fecha = Fecha::new(31, 12, 2026);
        assert_eq!(fecha.es_bisiesto(), false)
    }

    #[test]
    fn sumarFechaNoBisiesto(){
        let mut fecha = Fecha::new(31, 12, 2026);
        fecha.sumar_dias(7);
        assert_eq!(fecha.dd, 7);
        assert_eq!(fecha.mm, 1);
        assert_eq!(fecha.aaaa, 2027);
    }

    #[test]
    fn sumarFechaBisiesto(){
        let mut fecha = Fecha::new(28, 2, 2024);
        fecha.sumar_dias(7);
        assert_eq!(fecha.dd, 6);
        assert_eq!(fecha.mm, 3);
        assert_eq!(fecha.aaaa, 2024);
    }

    #[test]
    fn restarFechaNoBisiesto(){
        let mut fecha = Fecha::new(1, 1, 2026);
        fecha.restar_dias(31);
        assert_eq!(fecha.dd, 1);
        assert_eq!(fecha.mm, 12);
        assert_eq!(fecha.aaaa, 2025);
    }

    #[test]
    fn restarFechaBisiesto(){
        let mut fecha = Fecha::new(3, 3, 2024);
        fecha.restar_dias(4);
        assert_eq!(fecha.dd, 28);
        assert_eq!(fecha.mm, 2);
        assert_eq!(fecha.aaaa, 2024);
    }

    #[test]
    fn esFechaMayor(){
        let mut fechaA = Fecha::new(1, 1, 2026);
        let mut fechaB = Fecha::new(2, 1, 2026);
        assert_eq!(fechaA.es_mayor(fechaB), true);
    }

    #[test]
    fn noEsFechaMayor(){
        let mut fechaA = Fecha::new(1, 1, 2026);
        let mut fechaB = Fecha::new(1, 1, 2026);
        assert_eq!(fechaB.es_mayor(fechaA), false);
    }

    #[test]
    fn esFechaIgual(){
        let mut fechaA = Fecha::new(1, 1, 2026);
        let mut fechaB = Fecha::new(1, 1, 2026);
        assert_eq!(fechaB.es_mayor(fechaA), false);
    }

}