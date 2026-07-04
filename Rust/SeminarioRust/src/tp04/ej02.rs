use std::collections::LinkedList;

#[derive(Debug, Clone, PartialEq)]
struct Persona<'a>{
nombre:&'a str,
apellido:&'a str,
direccion:&'a str,
ciudad:&'a str,
salario:f64,
edad:u8,
}

impl<'a> Persona<'a>{
    pub fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str, ciudad:&'a str, salario:f64, edad:u8) -> Persona<'a>{
        Persona { nombre, apellido, direccion, ciudad, salario, edad }
}

}
pub fn funcion_a<'a>(personas: &Vec<Persona<'a>>, salario: f64) -> LinkedList<Persona<'a>>{
    personas.iter().filter(|x| x.salario > salario).cloned().collect()
}

pub fn funcion_b<'a>(personas: &Vec<Persona<'a>>, edad:u8, ciudad:&String) -> LinkedList<Persona<'a>>{
    personas.iter().filter(|x| x.edad > edad && x.ciudad.eq_ignore_ascii_case(&ciudad)).cloned().collect()
}

pub fn funcion_c<'a>(personas: &Vec<Persona<'a>>, ciudad:&String) -> bool{
    personas.iter().all(|x| x.ciudad.eq_ignore_ascii_case(&ciudad))
}

pub fn funcion_d<'a>(personas: &Vec<Persona<'a>>, ciudad:&String) -> bool{
    personas.iter().any(|x| x.ciudad.eq_ignore_ascii_case(&ciudad))
}

pub fn funcion_e<'a>(personas: &Vec<Persona<'a>>, persona:&Persona) -> bool{
    personas.iter().any(|x| x.eq(persona))
}

pub fn funcion_f<'a>(personas: &Vec<Persona<'a>>) -> LinkedList<u8>{
    personas.iter().map(|x| x.edad).collect()
}

pub fn funcion_g<'a>(personas: &Vec<Persona<'a>>) -> (Persona<'a>, Persona<'a>){
    let mut res:(Persona, Persona);
    let mut iter_p = personas.iter();
    let mut min: Persona = Persona::new("", "", "", "", 9999.9, 0);
    let mut max: Persona = Persona::new("", "", "", "", -9999.9, 0);

    for persona in iter_p{
        if persona.salario < min.salario{
            min = persona.clone();
        }
        if persona.salario > max.salario{
            max = persona.clone();
        }
    }
    res = (min, max);
    return res;
}


