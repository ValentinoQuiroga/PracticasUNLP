mod tp02;
mod tp03;
use tp03::ej01::Persona;

fn main() {
    let a: Persona = Persona::new("Valen".to_string(), 24, Some("Calle 10".to_string()));   
    println!("{}", a.to_string()); 
}
