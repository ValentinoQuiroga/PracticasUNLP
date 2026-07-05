use crate::{tp02::ej2::es_primo, tp04::ej01::cant_primos, tp03::ej7::*};

mod tp02;
mod tp03;
mod tp04;

fn main() {
    let mut con: ConsecionarioAuto = ConsecionarioAuto::new("Teueer".to_string(), "USA".to_string(), 5);
    let auto: Auto = Auto::new("A".to_string(), "A".to_string(), 2002, 100000.00, Color::Negro);
    con.agregar_auto(auto.clone());
    con.agregar_auto(auto.clone());
    con.agregar_auto(auto.clone());
    con.eliminar_auto(&auto);
}
