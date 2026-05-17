

pub fn es_primo(n: i32) -> bool{
    //Numeros enteros mayores a 1
    if n < 2 {
        panic!("El numero debe ser mayor a 1");
    }
    //Primer primo
    if n == 2 {
        return true;
    }
    //Ningun par es primo
    if n % 2 == 0 {
        return false;
    }

    //Pruebo multiplos hasta n * n
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn es_numero_primo(){
            assert_eq!(es_primo(23), true);
        }
    #[test]
        fn no_es_numero_primo(){
            assert_eq!(es_primo(24), false);
        }
    #[should_panic]
    #[test]
        fn panic_es_primo(){
            es_primo(-1);
        }
}