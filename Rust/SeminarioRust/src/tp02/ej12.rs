
pub fn reemplazar_pares(arreglo: &mut [i32;5]){
    for i in 0..5 {
        if arreglo[i] % 2 == 0{
            arreglo[i] = -1;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn reemplazando_dos_pares(){
        let mut arreglo = [1,2,3,4,5];
        let resultado_esperado = [1,-1,3,-1,5];
        reemplazar_pares(&mut arreglo);
        for i in 0..5 {
            assert_eq!(arreglo[i], resultado_esperado[i]);
        }
    }
}