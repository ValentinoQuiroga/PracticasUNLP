
pub fn suma_pares(arreglo: [u32; 5]) -> u32{
    let mut total = 0;
    for i in arreglo{   
        if i % 2 == 0{
            total += i;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn suma_pares_arrelo(){
            let arreglo = [1,2,3,4,5];
            assert_eq!(suma_pares(arreglo),6);
        }
}