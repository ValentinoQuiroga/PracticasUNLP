use std::collections::{VecDeque, btree_set::Difference};
use crate::tp03::{ej03::Fecha, ej08::Genero::OTROS, ej09::Animal::CABALLO};
struct Veterinaria{
    nombre: String,
    direccion: String,
    id: u32,
    cola: VecDeque<Mascota>,
    registros: VecDeque<Registro>
}
#[derive(Clone)]
struct Mascota{
    nombre: String,
    edad: u32,
    tipo: Animal,
    dueño: Dueño
}
#[derive(Clone)]
struct Dueño{
    nombre: String,
    direccion: String,
    telefono: String
}
#[derive(Clone)]
struct Registro{
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>
}
#[derive(Clone)]
enum Animal{
    PERRO,
    GATO,
    CABALLO,
    OTROS
}
impl Registro{
    fn new(mascota: Mascota, diagnostico: String, tratamiento: String, proxima_visita: Option<Fecha>) -> Registro{
        Registro { mascota, diagnostico, tratamiento, proxima_visita }
    }
    fn ig(&self, otro_registro: &Registro) -> bool{
        if !(self.mascota.ig(&otro_registro.mascota)) || (self.diagnostico != otro_registro.diagnostico) || (self.tratamiento != otro_registro.tratamiento){
            return false
        }else{
            match (&self.proxima_visita, &otro_registro.proxima_visita){
                (None, None) => true,
                (Some(fechaA), Some(fechaB)) => {
                    if fechaA.ig(&fechaB){ true}
                    else{ false}
                },
                _ => false
            }
        }
    }
}
impl Dueño{
    fn new(nombre: String, direccion: String, telefono: String) -> Dueño{
        Dueño { nombre, direccion, telefono }
    }
    fn ig(&self, otro_dueño: &Dueño) -> bool{
        if (self.nombre != otro_dueño.nombre)||(self.direccion != otro_dueño.direccion)||(self.telefono != otro_dueño.telefono){
            return false
        }else{ return true}
    }
}
impl Animal{
    fn ig(&self, otro_animal: &Animal) -> bool{
        match (self, otro_animal){
            (Animal::PERRO, Animal::PERRO) => true,
            (Animal::GATO, Animal::GATO) => true,
            (Animal::CABALLO, Animal::CABALLO) => true,
            (Animal::OTROS, Animal::OTROS) => true,
            _ => false
        }
    }
}
impl Mascota{
    fn new(nombre: String, edad: u32, tipo: Animal, dueño: Dueño) -> Mascota{
        Mascota { nombre, edad, tipo, dueño }
    }
    fn ig(&self, otra_mascota: &Mascota) -> bool{
        if (self.nombre != otra_mascota.nombre)||(self.edad != otra_mascota.edad)||!(self.dueño.ig(&otra_mascota.dueño))||!(self.tipo.ig(&otra_mascota.tipo)){
            return false
        }else{ return true}
    }
}
impl Veterinaria{
    fn new(nombre: String, direccion: String, id: u32) -> Veterinaria{
        let cola: VecDeque<Mascota> = VecDeque::new();
        let registros: VecDeque<Registro> = VecDeque::new();
        Veterinaria { nombre, direccion, id, cola, registros}
    }
    fn agregar_nueva_mascota(&mut self, mascota: Mascota){
        self.cola.push_back(mascota);
    }
    fn agregar_nueva_mascota_maxima_prioridad(&mut self, mascota: Mascota){
        self.cola.push_front(mascota);
    }
    fn atender_mascota(&mut self) -> Option<Mascota>{
        return self.cola.pop_front();
    }
    fn eliminar_mascota_de_la_cola(&mut self, mascota: &Mascota){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while (pos < self.cola.len()) && !(encontrado){
            if self.cola[pos].ig(mascota){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            self.cola.remove(pos);
        }
    }
    fn registrar_atencion(&mut self, atencion: Registro){
        self.registros.push_front(atencion);
    }
    fn buscar_atencion(&self, nombre_mascota: String, nombre_dueño: String, telefono: String) -> VecDeque<&Registro>{
        let mut atenciones: VecDeque<&Registro> = VecDeque::new();

       for i in 0..self.registros.len(){
            let aux = &self.registros[i].mascota;
            if (aux.nombre == nombre_mascota) && (aux.dueño.nombre == nombre_dueño) && (aux.dueño.telefono == telefono){
                    atenciones.push_back(&self.registros[i]);
            }
        }
        return atenciones
    }
    fn modificar_diagnostico(&mut self, atencion: &Registro, modificacion: String){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while (pos < self.registros.len()) && !(encontrado){
            if self.registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            self.registros[pos].diagnostico = modificacion;
        }
    }
    fn modificar_proxima_visita(&mut self, atencion: &Registro, nueva_fecha: Fecha){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while (pos < self.registros.len()) && !(encontrado){
            if self.registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            self.registros[pos].proxima_visita = Some(nueva_fecha);
        }
    }
    fn eliminar_atencion(&mut self, atencion: &Registro){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;

        while (pos < self.registros.len()) && !(encontrado){
            if self.registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            self.registros.remove(pos);
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_funciones_ig_datos_iguales(){
        let dueñoA = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let dueñoB = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        assert_eq!(dueñoA.ig(&dueñoB), true);

        let animalA: Animal = Animal::GATO;
        let animalB: Animal = Animal::GATO;
        assert_eq!(animalA.ig(&animalB), true);

        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);
        let mascotaB: Mascota = Mascota::new("A".to_string(), 5, animalB, dueñoB);
        assert_eq!(mascotaA.ig(&mascotaB), true);

        let fechaA: Fecha = Fecha::new(16, 04, 2002);
        let fechaB: Fecha = Fecha::new(16, 04, 2002);
        assert_eq!(fechaA.ig(&fechaB), true);

        let registroA: Registro = Registro::new(mascotaA, "A".to_string(), "A".to_string(), Some(fechaA));
        let registroB: Registro = Registro::new(mascotaB, "A".to_string(), "A".to_string(), Some(fechaB));
        assert_eq!(registroA.ig(&registroB), true);
    }
    #[test]
    fn test_funciones_ig_datos_distintos(){
        let dueñoA = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let dueñoB = Dueño::new("B".to_string(), "10".to_string(), "1111".to_string());
        assert_ne!(dueñoA.ig(&dueñoB), true);

        let animalA: Animal = Animal::GATO;
        let animalB: Animal = Animal::PERRO;
        assert_ne!(animalA.ig(&animalB), true);

        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);
        let mascotaB: Mascota = Mascota::new("A".to_string(), 5, animalB, dueñoB);
        assert_ne!(mascotaA.ig(&mascotaB), true);

        let fechaA: Fecha = Fecha::new(16, 04, 2002);
        let fechaB: Fecha = Fecha::new(15, 04, 2002);
        assert_ne!(fechaA.ig(&fechaB), true);

        let registroA: Registro = Registro::new(mascotaA, "A".to_string(), "A".to_string(), Some(fechaA));
        let registroB: Registro = Registro::new(mascotaB, "A".to_string(), "A".to_string(), Some(fechaB));
        assert_ne!(registroA.ig(&registroB), true);
    }
    #[test]
    fn test_comparacion_registros_sin_fechas(){
        let fecha: Fecha = Fecha::new(16, 04, 2002);

        let dueñoA = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let animalA: Animal = Animal::GATO;
        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);

        let dueñoB = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let animalB: Animal = Animal::GATO;
        let mascotaB: Mascota = Mascota::new("A".to_string(), 5, animalB, dueñoB);

        let mut registroA: Registro = Registro::new(mascotaA, "A".to_string(), "A".to_string(), None);
        let registroB: Registro = Registro::new(mascotaB, "A".to_string(), "A".to_string(), None);
        
        assert_eq!(registroA.ig(&registroB), true);

        registroA.proxima_visita = Some(fecha);

        assert_ne!(registroA.ig(&registroB), true);
    }

    #[test]
    fn test_cola_normal(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(), "10".to_string(), 3);
        
        let dueñoA = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animalA: Animal = Animal::GATO;
        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);

        let dueñoB = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animalB: Animal = Animal::PERRO;
        let mascotaB: Mascota = Mascota::new("B".to_string(), 5, animalB, dueñoB);

        let dueñoC = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animalC: Animal = Animal::GATO;
        let mascotaC: Mascota = Mascota::new("C".to_string(), 5, animalC, dueñoC);

        vet.agregar_nueva_mascota(mascotaA.clone());
        vet.agregar_nueva_mascota(mascotaB.clone());
        vet.agregar_nueva_mascota(mascotaC.clone());

        assert_eq!(vet.cola.len(), 3);

        let mut mascota_atendida: Mascota = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaA), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaB), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaC), true);

        assert_eq!(vet.cola.len(), 0);
    }
    #[test]
    fn test_urgencia(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(), "10".to_string(), 3);
        
        let dueñoA = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animalA: Animal = Animal::GATO;
        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);

        let dueñoB = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animalB: Animal = Animal::PERRO;
        let mascotaB: Mascota = Mascota::new("B".to_string(), 5, animalB, dueñoB);

        let dueñoC = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animalC: Animal = Animal::GATO;
        let mascotaC: Mascota = Mascota::new("C".to_string(), 5, animalC, dueñoC);

        vet.agregar_nueva_mascota(mascotaA.clone());
        vet.agregar_nueva_mascota(mascotaB.clone());
        vet.agregar_nueva_mascota_maxima_prioridad(mascotaC.clone());

        let mut mascota_atendida: Mascota = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaC), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaA), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascotaB), true);

    }
    #[test]
    fn test_eliminar_mascota_de_cola(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(), "10".to_string(), 3);
        
        let dueñoA = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animalA: Animal = Animal::GATO;
        let mascotaA: Mascota = Mascota::new("A".to_string(), 5, animalA, dueñoA);

        let dueñoB = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animalB: Animal = Animal::PERRO;
        let mascotaB: Mascota = Mascota::new("B".to_string(), 5, animalB, dueñoB);

        let dueñoC = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animalC: Animal = Animal::GATO;
        let mascotaC: Mascota = Mascota::new("C".to_string(), 5, animalC, dueñoC);

        vet.agregar_nueva_mascota(mascotaA.clone());
        vet.agregar_nueva_mascota(mascotaB.clone());
        vet.agregar_nueva_mascota(mascotaC.clone());

        vet.eliminar_mascota_de_la_cola(&mascotaB);

        let mut mascota_atendida: Option<Mascota> = vet.atender_mascota();
        assert_eq!(mascota_atendida.unwrap().ig(&mascotaA), true);

        mascota_atendida = vet.atender_mascota();
        assert_eq!(mascota_atendida.unwrap().ig(&mascotaC), true);

        mascota_atendida = vet.atender_mascota();
        assert_eq!(mascota_atendida.is_none(), true);
    }

}