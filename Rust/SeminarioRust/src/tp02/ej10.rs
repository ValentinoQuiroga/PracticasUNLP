
pub fn cantidad_de_cadenas_mayores_a(arreglo: [String; 5], limite: usize) -> u32{
    let mut cant = 0;
    for i in arreglo{
        if i.chars().count() > limite{
            cant += 1;
        }
    }
    cant
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn tres_cadenas_mayores_a_cinco(){
        let a: [String; 5] = ["".to_string(),"12345".to_string(),"123456".to_string(),"1234567".to_string(),"12345678".to_string()];
        let l = 5;
        assert_eq!(cantidad_de_cadenas_mayores_a(a, l), 3);
    }
}