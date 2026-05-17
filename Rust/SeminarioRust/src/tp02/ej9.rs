
pub fn cantidad_en_rango(arreglo: [u32;5], inferior: u32, superior:u32) -> u32{
    let mut cant = 0;
    for i in arreglo{
        if i >= inferior && i <= superior {
            cant += 1;
        } 
    }
    cant
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn tres_en_rango(){
        let a = [1,2,3,4,5];
        let i = 2;
        let s = 4;
        assert_eq!(cantidad_en_rango(a,i,s), 3);
    }
}