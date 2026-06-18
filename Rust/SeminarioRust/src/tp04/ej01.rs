use crate::tp02::ej2::es_primo;


pub trait Primo {
    fn es_primo(&self) -> bool;
}

impl Primo for u16 {
    fn es_primo(&self) -> bool{
        let mut c: u8 = 0;
        for i in 1..100{
            if (self % i) == 0{
                c += 1;
            }
            if (c > 2){
                break}
        }
        return (c < 3);
    }
}
pub fn cant_primos(nums: Vec<u16>) -> u8{
    let mut c = 0;
    let mut ns = nums.iter();
    while let Some(n) = ns.next(){
        if n.es_primo() { c += 1}
    }
    c
}

#[cfg(test)]

mod tests{
    use super::*; 

    #[test]
    fn test_vec_7_primos(){
        let mut vec = Vec::new();
        for i in 1..14{
            vec.push(i);
        }

        assert_eq!(cant_primos(vec), 7);
    }
}