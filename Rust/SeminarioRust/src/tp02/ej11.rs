
pub fn multiplicar_valores(arreglo: &mut[u32], factor: u32){ //recibo la referencia mutable del arreglo
    for i in 0..5{ 
        arreglo[i] = arreglo[i] * factor;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn arreglo_multiplicado_por_cinco(){
        let mut arreglo = [1,2,3,4,5];
        let resultado_esperado = [5,10,15,20,25];
        let f = 5;
        multiplicar_valores(&mut arreglo,f);
        assert_eq!(arreglo, resultado_esperado);
    }
}