//Usar usize o cambiar el tipo?
pub fn longitud_de_cadenas(arreglo: [String; 5]) -> [usize; 5]{ 
    let mut resultado: [usize;5] = [0,0,0,0,0];
    for i in 0..5{
        resultado[i] = arreglo[i].chars().count();
    }
    resultado
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn longitud_de_cadenas_comparacion_correcta(){
        let arreglo:[String; 5] = ["a".to_string(),"ab".to_string(),"abc".to_string(),"abcd".to_string(),"abcde".to_string()];
        let resultado = longitud_de_cadenas(arreglo);
        assert_eq!(resultado[0], 1);
        assert_eq!(resultado[1], 2);
        assert_eq!(resultado[2], 3);
        assert_eq!(resultado[3], 4);
        assert_eq!(resultado[4], 5);
    }
    #[test]
    fn longitud_de_cadenas_comparacion_incorrecta(){
        let arreglo:[String; 5] = ["a".to_string(),"ab".to_string(),"abc".to_string(),"abcd".to_string(),"abcde".to_string()];
        let resultado = longitud_de_cadenas(arreglo);
        assert_ne!(resultado[0], 0);
        assert_ne!(resultado[1], 0);
        assert_ne!(resultado[2], 0);
        assert_ne!(resultado[3], 0);
        assert_ne!(resultado[4], 0);
    }
}