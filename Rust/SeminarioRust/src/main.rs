mod tp02;

fn main() {
    
}

#[test]
    fn es_numero_par(){
        assert_eq!(tp02::ej1::es_par(6), true);
    }
#[test]
    fn no_es_numero_par(){
        assert_eq!(tp02::ej1::es_par(7), false);
    }
#[test]
    fn es_numero_primo(){
        assert_eq!(tp02::ej2::es_primo(23), true);
    }
#[test]
    fn no_es_numero_primo(){
        assert_eq!(tp02::ej2::es_primo(24), false);
    }
#[should_panic]
#[test]
    fn panic_es_primo(){
        tp02::ej2::es_primo(-1);
    }
#[test]
    fn suma_pares_arrelo(){
        let arreglo = [1,2,3,4,5];
        assert_eq!(tp02::ej3::suma_pares(arreglo),6);
    }