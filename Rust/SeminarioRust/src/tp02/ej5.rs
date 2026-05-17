
pub fn duplicar_valores(arreglo: [u32; 5]) -> [u32; 5]{
    let mut resultado: [u32; 5] = [0,0,0,0,0];
    for i in 0..5{
        resultado[i] = arreglo[i] * 2;
    }
    resultado
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn duplicar_valores_estandar(){
        let arreglo = [1,2,3,4,5];
        let duplicado = duplicar_valores(arreglo);
        let mut es_duplicado = false;
        for i in 0..5{
            if arreglo[i] * 2 == duplicado[i]{
                es_duplicado = true;
            }
        }

        assert_eq!(es_duplicado, true);
    }

}