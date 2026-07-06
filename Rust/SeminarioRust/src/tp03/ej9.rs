use std::{collections::VecDeque, io::{BufReader, Error}, path};
use crate::tp03::ej3::Fecha;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};

struct Veterinaria{
    nombre: String,
    direccion: String,
    id: u32,
    cola: VecDeque<Mascota>,
    registros: VecDeque<Registro>,
    path: String
}
#[derive(Clone, Serialize, Deserialize)]
struct Mascota{
    nombre: String,
    edad: u8,
    tipo: Animal,
    dueño: Dueño
}
#[derive(Clone, Serialize, Deserialize)]
struct Dueño{
    nombre: String,
    direccion: String,
    telefono: String
}
#[derive(Clone, Serialize, Deserialize)]
struct Registro{
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>
}
#[derive(Clone, Serialize, Deserialize)]
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
                (Some(fecha_a), Some(fecha_b)) => {
                    if fecha_a.ig(&fecha_b){ true}
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
    fn new(nombre: String, edad: u8, tipo: Animal, dueño: Dueño) -> Mascota{
        Mascota { nombre, edad, tipo, dueño }
    }
    fn ig(&self, otra_mascota: &Mascota) -> bool{
        if (self.nombre != otra_mascota.nombre)||(self.edad != otra_mascota.edad)||!(self.dueño.ig(&otra_mascota.dueño))||!(self.tipo.ig(&otra_mascota.tipo)){
            return false
        }else{ return true}
    }
}
impl Veterinaria{
    pub fn actualizar_informe(&self, registros: VecDeque<Registro>) -> Result<File, Error>{
        let registros_serializados = serde_json::to_string_pretty(&registros)?;
        let result = File::create(&self.path);
        match result{
            Err(e) => Err(e),
            Ok(mut archivo) => {
                archivo.write_all(&registros_serializados.as_bytes())?;
                Ok(archivo)}
        }
    }
    fn new(nombre: String, direccion: String, id: u32, path: String) -> Veterinaria{
        let cola: VecDeque<Mascota> = VecDeque::new();
        let registros: VecDeque<Registro> = VecDeque::new();
        if let Ok(mut archivo) = File::create(&path){
            let _ = archivo.write_all("[]".as_bytes());
        }
        Veterinaria { nombre, direccion, id, cola, registros, path}
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
    fn obtener_registros(&self) -> Result<VecDeque<Registro>, Error>{;
        let archivo = match File::open(&self.path){
            Ok(f) => f,
            Err(_) => return Ok(VecDeque::new()),
        };

        let lector = BufReader::new(archivo);
        
        let mut lista_deserializada: 
            VecDeque<Registro> = serde_json::from_reader(lector).unwrap_or_default();
        return Ok(lista_deserializada);

    }
    fn registrar_atencion(&self, atencion: Registro) -> Result<(), Error>{
        let mut registros = self.obtener_registros()?;
        registros.push_front(atencion);
        self.actualizar_informe(registros);
        return Ok(());
    }
    fn buscar_atencion(&self, nombre_mascota: &String, nombre_dueño: &String, telefono: &String) -> Option<Registro>{
        let mut atencion: Option<Registro> = None;
        let result_registros = self.obtener_registros();
        let registros: VecDeque<Registro>;
        match result_registros{
            Err(_) => return None,
            Ok(r) => registros = r
        }
        let mut i = 0;
        let mut encontrado = false;

       while( i < registros.len()) && (!encontrado){
            let registro_aux = &registros[i].mascota;
            if (registro_aux.nombre == *nombre_mascota) && (registro_aux.dueño.nombre == *nombre_dueño) && (registro_aux.dueño.telefono == *telefono){
                    atencion = Some(registros[i].clone());
                    encontrado = true;
            }else{
                i += 1;
            }
        }
        return atencion;
    }
    fn modificar_diagnostico(&mut self, atencion: &Registro, modificacion: String){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;
        let mut registros = self.obtener_registros().unwrap();

        while (pos < registros.len()) && !(encontrado){
            if registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            registros[pos].diagnostico = modificacion;
            self.actualizar_informe(registros);
        }

    }
    fn modificar_proxima_visita(&mut self, atencion: &Registro, nueva_fecha: Fecha){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;
        let mut registros = self.obtener_registros().unwrap();

        while (pos < registros.len()) && !(encontrado){
            if registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            registros[pos].proxima_visita = Some(nueva_fecha);
            self.actualizar_informe(registros);
        }
    }
    fn eliminar_atencion(&mut self, atencion: &Registro){
        let mut pos: usize = 0;
        let mut encontrado: bool = false;
        let mut registros = self.obtener_registros().unwrap();

        while (pos < registros.len()) && !(encontrado){
            if registros[pos].ig(atencion){
                encontrado = true;
            }else{ pos += 1}
        }

        if encontrado{
            registros.remove(pos);
            self.actualizar_informe(registros);
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_funciones_ig_datos_iguales(){
        let dueño_a = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let dueño_b = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        assert_eq!(dueño_a.ig(&dueño_b), true);

        let animal_a: Animal = Animal::GATO;
        let animal_b: Animal = Animal::GATO;
        assert_eq!(animal_a.ig(&animal_b), true);

        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);
        let mascota_b: Mascota = Mascota::new("A".to_string(), 5, animal_b, dueño_b);
        assert_eq!(mascota_a.ig(&mascota_b), true);

        let fecha_a: Fecha = Fecha::new(16, 04, 2002);
        let fecha_b: Fecha = Fecha::new(16, 04, 2002);
        assert_eq!(fecha_a.ig(&fecha_b), true);

        let registro_a: Registro = Registro::new(mascota_a, "A".to_string(), "A".to_string(), Some(fecha_a));
        let registro_b: Registro = Registro::new(mascota_b, "A".to_string(), "A".to_string(), Some(fecha_b));
        assert_eq!(registro_a.ig(&registro_b), true);
    }
    #[test]
    fn test_funciones_ig_datos_distintos(){
        let dueño_a = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let dueño_b = Dueño::new("B".to_string(), "10".to_string(), "1111".to_string());
        assert_ne!(dueño_a.ig(&dueño_b), true);

        let animal_a: Animal = Animal::GATO;
        let animal_b: Animal = Animal::PERRO;
        assert_ne!(animal_a.ig(&animal_b), true);

        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);
        let mascota_b: Mascota = Mascota::new("A".to_string(), 5, animal_b, dueño_b);
        assert_ne!(mascota_a.ig(&mascota_b), true);

        let fecha_a: Fecha = Fecha::new(16, 04, 2002);
        let fecha_b: Fecha = Fecha::new(15, 04, 2002);
        assert_ne!(fecha_a.ig(&fecha_b), true);

        let registro_a: Registro = Registro::new(mascota_a, "A".to_string(), "A".to_string(), Some(fecha_a));
        let registro_b: Registro = Registro::new(mascota_b, "A".to_string(), "A".to_string(), Some(fecha_b));
        assert_ne!(registro_a.ig(&registro_b), true);
    }
    #[test]
    fn test_comparacion_registros_sin_fechas(){
        let fecha: Fecha = Fecha::new(16, 04, 2002);

        let dueño_a = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);

        let dueño_b = Dueño::new("A".to_string(), "10".to_string(), "1111".to_string());
        let animal_b: Animal = Animal::GATO;
        let mascota_b: Mascota = Mascota::new("A".to_string(), 5, animal_b, dueño_b);

        let mut registro_a: Registro = Registro::new(mascota_a, "A".to_string(), "A".to_string(), None);
        let registro_b: Registro = Registro::new(mascota_b, "A".to_string(), "A".to_string(), None);
        
        assert_eq!(registro_a.ig(&registro_b), true);

        registro_a.proxima_visita = Some(fecha);

        assert_eq!(registro_a.ig(&registro_b), false);
    }

    #[test]
    fn test_cola_normal(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3, "src/tp03/tests/ej9/test_cola_normal.json".to_string());
        
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);

        let dueño_b = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animal_b: Animal = Animal::PERRO;
        let mascota_b: Mascota = Mascota::new("B".to_string(), 5, animal_b, dueño_b);

        let dueño_c = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animal_c: Animal = Animal::GATO;
        let mascota_c: Mascota = Mascota::new("C".to_string(), 5, animal_c, dueño_c);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.agregar_nueva_mascota(mascota_b.clone());
        vet.agregar_nueva_mascota(mascota_c.clone());

        assert_eq!(vet.cola.len(), 3);

        let mut mascota_atendida: Mascota = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_a), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_b), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_c), true);

        assert_eq!(vet.cola.len(), 0);
    }
    #[test]
    fn test_urgencia(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_urgencia.json".to_string());
        
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);

        let dueño_b = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animal_b: Animal = Animal::PERRO;
        let mascota_b: Mascota = Mascota::new("B".to_string(), 5, animal_b, dueño_b);

        let dueño_c = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animal_c: Animal = Animal::GATO;
        let mascota_c: Mascota = Mascota::new("C".to_string(), 5, animal_c, dueño_c);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.agregar_nueva_mascota(mascota_b.clone());
        vet.agregar_nueva_mascota_maxima_prioridad(mascota_c.clone());

        let mut mascota_atendida: Mascota = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_c), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_a), true);

        mascota_atendida = vet.atender_mascota().unwrap();
        assert_eq!(mascota_atendida.ig(&mascota_b), true);

    }
    #[test]
    fn test_eliminar_mascota_de_cola(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_eliminar_mascota_de_cola.json".to_string());        
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a);

        let dueño_b = Dueño::new("B".to_string(), "B".to_string(), "1111".to_string());
        let animal_b: Animal = Animal::PERRO;
        let mascota_b: Mascota = Mascota::new("B".to_string(), 5, animal_b, dueño_b);

        let dueño_c = Dueño::new("C".to_string(), "C".to_string(), "2222".to_string());
        let animal_c: Animal = Animal::GATO;
        let mascota_c: Mascota = Mascota::new("C".to_string(), 5, animal_c, dueño_c);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.agregar_nueva_mascota(mascota_b.clone());
        vet.agregar_nueva_mascota(mascota_c.clone());

        vet.eliminar_mascota_de_la_cola(&mascota_b);

        let mut mascota_atendida: Option<Mascota> = vet.atender_mascota();
        assert_eq!(mascota_atendida.unwrap().ig(&mascota_a), true); //Se atiende mascota A

        mascota_atendida = vet.atender_mascota();
        assert_eq!(mascota_atendida.unwrap().ig(&mascota_c), true); //Se atiende mascota C

        mascota_atendida = vet.atender_mascota();
        assert_eq!(mascota_atendida.is_none(), true); //Se elimino mascota B, por lo que no hay 3er mascota atendida
    }
    
    #[test]
    fn test_modificar_proxima_visita(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_modificar_proxima_visita.json".to_string());        
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a.clone());
        let registro = Registro::new(mascota_a.clone(), "Diagnostico A".to_string(), "A".to_string(), None);

        assert_eq!(registro.proxima_visita.is_none(), true);

        vet.agregar_nueva_mascota(mascota_a.clone());
        let nueva_fecha: Fecha = Fecha::new(16, 04, 2026);
        vet.registrar_atencion(registro.clone());
        vet.modificar_proxima_visita(&registro, nueva_fecha.clone());

        let fecha_modificada = vet
            .buscar_atencion(&"A".to_string(), 
            &"A".to_string(), &"0000".to_string()).unwrap()
            .proxima_visita;
        assert_eq!(nueva_fecha.ig(&fecha_modificada.unwrap()), true);
    }

    #[test]
    fn test_modificar_diagnostico(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_modificar_diagnostico.json".to_string());
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a.clone());
        let registro = Registro::new(mascota_a.clone(), "Diagnostico A".to_string(), "A".to_string(), None);

        assert_eq!(registro.proxima_visita.is_none(), true);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.registrar_atencion(registro.clone());
        vet.modificar_diagnostico(&registro, "Diagnostico B".to_string());

        let mut diagnostico_modificado = registro.clone();
        diagnostico_modificado.diagnostico = "Diagnostico B".to_string();

        assert_eq!(vet.buscar_atencion(&"A".to_string(), &"A".to_string(), &"0000".to_string()).unwrap().ig(&diagnostico_modificado), true);
    }

    #[test]
    fn test_buscar_atencion(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_buscar_atencion.json".to_string());
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a, dueño_a.clone());
        let registro = Registro::new(mascota_a.clone(), "Diagnostico A".to_string(), "A".to_string(), None);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.registrar_atencion(registro.clone());
        vet.registrar_atencion(registro.clone());
        vet.registrar_atencion(registro.clone());  
        let atencion = vet.buscar_atencion(&"A".to_string(),
             &"A".to_string(), &"0000".to_string());
        assert_eq!(atencion.unwrap().ig(&registro), true); 
    }

    #[test]
    fn test_eliminar_atencion(){
        let mut vet: Veterinaria = Veterinaria::new("Vet".to_string(),
         "10".to_string(), 3,"src/tp03/tests/ej9/test_eliminar_atencion.json".to_string());
        let dueño_a = Dueño::new("A".to_string(), "A".to_string(), "0000".to_string());
        let animal_a: Animal = Animal::GATO;
        let mascota_a: Mascota = Mascota::new("A".to_string(), 5, animal_a.clone(), dueño_a.clone());
        let mascota_b: Mascota = Mascota::new("B".to_string(), 5, animal_a.clone(), dueño_a.clone());
        let registro = Registro::new(mascota_a.clone(), "Diagnostico A".to_string(), "A".to_string(), None);
        let registro_b: Registro = Registro::new(mascota_b.clone(), "Diagnostico A".to_string(), "A".to_string(), None);

        vet.agregar_nueva_mascota(mascota_a.clone());
        vet.registrar_atencion(registro.clone());
        vet.registrar_atencion(registro.clone());
        vet.registrar_atencion(registro_b.clone());
        vet.eliminar_atencion(&registro_b);
        
        
        let atencion = vet.buscar_atencion(&"B".to_string(),
             &"A".to_string(), &"0000".to_string());
        
        assert_eq!(atencion.is_none(), true);

    }
}