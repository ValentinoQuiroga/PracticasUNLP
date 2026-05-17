
pub fn sumar_arreglo(arreglo1: [f32; 5], arreglo2: [f32; 5]) -> [f32; 5]{
    let mut arreglo3 : [f32;5] = [0.0,0.0,0.0,0.0,0.0];

    for i in 0..5{
        arreglo3[i] = arreglo1[i] + arreglo2[i];
    }
    return arreglo3;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn suma_de_arreglos(){
        let a1:[f32; 5] = [1.1,2.2,3.3,4.4,5.5];
        let a2:[f32; 5] = [1.5,2.4,3.3,4.2,5.1];
        let a3:[f32; 5] = sumar_arreglo(a1, a2);

        for i in 0..5{
            assert_eq!(a3[i], a1[i] + a2[i]);
        }
    }
}
