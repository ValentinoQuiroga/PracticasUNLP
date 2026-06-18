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




