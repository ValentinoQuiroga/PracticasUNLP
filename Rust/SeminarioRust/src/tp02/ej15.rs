
pub fn serie_geometrica<const TAMAÑO: usize>() -> [usize; TAMAÑO]{
    let mut arreglo: [usize; TAMAÑO] = [0;TAMAÑO];
    for i in 0..TAMAÑO{
    let mut valor:usize = 1;
        for _j in 0.. i{
            valor = valor * 2;
        }
        arreglo[i] = valor ;
    }
    arreglo
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn serie_geometrica_tamaño_9(){
        let esperado = [1,2,4,8,16,32,64,128,256];
        let arreglo: [usize; 9] = serie_geometrica();

        assert_eq!(arreglo,esperado);
    }
}