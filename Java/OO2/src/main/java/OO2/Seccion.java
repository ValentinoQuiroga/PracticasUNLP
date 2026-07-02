package OO2;
import java.util.*;

public class Seccion implements Elemento{
	private ArrayList<Elemento> coleccion;
	private String titulo;
	private Translator t;
	
	public Seccion(String titulo) {
		this.titulo = titulo;
		this.coleccion = new ArrayList<Elemento>();
		this.t = new Translator();
	}
	
	public void agregarElemento(Elemento elemento) {
		coleccion.add(elemento);
	}
	
	public String toString() {
		String texto = "###" + this.titulo + "\n";
		for (Elemento e: coleccion) {
			texto += e.toString();
		}
		return texto;
	}
	
	public boolean buscar(String texto) {
		boolean encontrado = this.titulo.matches(texto);
		int i = 0;
		while ((!encontrado) && (i < coleccion.size())){
			encontrado = coleccion.get(i).buscar(texto);
			i++;
		}
		return encontrado;
	}

	public Elemento traducir() {
		String tituloTraducido = t.translate(titulo);
		Seccion seccionTraducida = new Seccion(tituloTraducido);
		for (Elemento e: coleccion) {
			seccionTraducida.agregarElemento(e.traducir());
		}
		return seccionTraducida;
	}
}
