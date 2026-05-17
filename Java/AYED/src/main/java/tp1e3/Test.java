package tp1e3;

public class Test {

	public static void main(String[] args) {
		Estudiante [] estudiantes = new Estudiante[2];
		for (int i = 0; i < 2; i++) {
			estudiantes[i] = new Estudiante();
		}
		Profesor [] profesores = new Profesor[3];
		for (int i = 0; i < 3; i++) {
			profesores[i] = new Profesor();
		}
		estudiantes[0].setApellido("Quiroga");
		estudiantes[0].setNombre("Valentino");
		estudiantes[0].setComision("A");
		estudiantes[0].setDireccion("C10");
		estudiantes[0].setEmail("qga");

		estudiantes[1].setApellido("Muñoz");
		estudiantes[1].setNombre("Abril");
		estudiantes[1].setComision("B");
		estudiantes[1].setDireccion("1668");
		estudiantes[1].setEmail("mrrz");

		profesores[0].setApellido("A");
		profesores[0].setNombre("A");
		profesores[0].setCatedra("A");
		profesores[0].setEmail("A");
		profesores[0].setFacultad("A");

		profesores[1].setApellido("B");
		profesores[1].setNombre("B");
		profesores[1].setCatedra("B");
		profesores[1].setEmail("B");
		profesores[1].setFacultad("B");
		
		profesores[2].setApellido("C");
		profesores[2].setNombre("C");
		profesores[2].setCatedra("C");
		profesores[2].setEmail("C");
		profesores[2].setFacultad("C");

		for (int i = 0; i < 2; i++) {
			System.out.println(estudiantes[i].tusDatos());
		}
		for (int i = 0; i < 3; i++) {
			System.out.println(profesores[i].tusDatos());
		}
	}

}
