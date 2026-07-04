
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
}

#[cfg(test)]
mod tests{
    use super::*;

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