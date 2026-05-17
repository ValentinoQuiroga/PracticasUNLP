
pub fn incrementar(n: &mut f32){
    *n += 1.0;
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn incrementar_estandar(){
        let mut f: f32 = 1.5;
        incrementar(&mut f);
        assert_eq!(f,2.5);
    }
}