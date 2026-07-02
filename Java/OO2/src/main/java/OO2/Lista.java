package OO2;
import java.util.*;

public class Lista extends Texto{
	private ArrayList<String> lista;
	private Translator t;
	
	public Lista(String titulo) {
		super(titulo);
		this.lista = new ArrayList<String>();
		this.t = new Translator();
	}
	
	public void agregarInciso(String inciso) {
		lista.add(inciso);
	}
	public String toString() {
		String texto =  super.toString();
		for (int i = 0; i < lista.size(); i++){
			texto += i + ": " + lista.get(i) + "\n";
		}
		return texto;
	}
	
	public Elemento traducir() {
		String tituloTraducido = t.translate(getTitulo());
		Lista listaTraducida = new Lista(tituloTraducido);
		for (String i: lista) {
			listaTraducida.agregarInciso(t.translate(i));
		}
		return listaTraducida;
	}
	
	@Override
	public boolean buscar(String texto) {
		boolean encontrado = super.buscar(texto);
		int i = 0;
		
		while ((!encontrado) && (i < lista.size())) {
			encontrado = lista.get(i).matches(texto);
			i++;
		}
		return encontrado;
	}
}
