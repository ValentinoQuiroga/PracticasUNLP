
pub fn ordenar_nombres(arreglo: &mut[String; 5]){
    arreglo.sort();
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn ordenar_nombres_estandar(){
        let mut arreglo = ["ba".to_string(),"zz".to_string(),"a".to_string(),"x".to_string(),"be".to_string()];
        let esperado = ["a".to_string(),"ba".to_string(),"be".to_string(),"x".to_string(),"zz".to_string()];
        ordenar_nombres(&mut arreglo);
        for i in 0..5{
            assert_eq!(arreglo[i], esperado[i]);
        }
    }
}