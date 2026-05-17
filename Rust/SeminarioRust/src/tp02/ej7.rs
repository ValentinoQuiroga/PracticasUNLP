
pub fn cantidad_de_mayores(arreglo: [u32; 5], limite: u32) -> u32{
    let mut cant: u32 = 0;

    for i in arreglo{
        if i > limite{
            cant+= 1;
        }
    }
    cant
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn tres_mayores(){
        let arreglo = [1,2,3,4,5];
        let limite = 2;
        assert_eq!(cantidad_de_mayores(arreglo, limite), 3);
    }
    #[test]
    fn ningun_mayor(){
        let arreglo = [1,2,3,4,5];
        let limite = 5;
        assert_eq!(cantidad_de_mayores(arreglo, limite), 0);
    }
}