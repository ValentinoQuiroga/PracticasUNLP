package OO2;

public class Prueba {

	public static void main(String[] args) {
		Seccion raiz = new Seccion("Cosas importantes");
		Parrafo parr = new Parrafo("Aviso", "A continuacion detallare los pasos mas importantes para tejer como una pupita");
		Lista lista = new Lista("Cosas necesarias");
		lista.agregarInciso("Lana bonita");
		lista.agregarInciso("Agujas de tamaño pupita");
		lista.agregarInciso("Muchas ganas de hacer bufandas");
		raiz.agregarElemento(parr);
		raiz.agregarElemento(lista);
		
		Seccion sec1 = new Seccion("Saberes");
		Lista lista1 = new Lista("Puntos conocidos");
		lista1.agregarInciso("Punto comun");
		lista1.agregarInciso("Punto revez");
		lista1.agregarInciso("Punto jersey");
		sec1.agregarElemento(lista1);
		raiz.agregarElemento(sec1);
		
		Documento doc = new Documento("Como tejer", "Pupita", raiz);
		
		System.out.println(doc.toString());
		
		System.out.println(doc.buscar("Pupita"));
		
		Documento docTraducido = doc.traducir();
		
		System.out.println(docTraducido.toString());
	}

}
