
pub fn cantidad_impares(arreglo: [u32; 5]) -> u32{
    let mut cant = 0;
    for i in arreglo{
        if i % 2 > 0{
            cant+= 1;
        }
    }
    cant
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn cantidad_impares_tres(){
        let arreglo = [1,2,3,4,5];
        assert_eq!(cantidad_impares(arreglo), 3);
    }
}