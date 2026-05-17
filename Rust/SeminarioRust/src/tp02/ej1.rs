
pub fn es_par(i: u32) -> bool{
    return (i % 2) == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn es_numero_par(){
            assert_eq!(es_par(6), true);
        }
    #[test]
        fn no_es_numero_par(){
            assert_eq!(es_par(7), false);
        }
}