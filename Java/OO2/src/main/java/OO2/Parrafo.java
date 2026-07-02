package OO2;

public class Parrafo extends Texto{
	private String parrafo;
	private Translator t;
	
	public Parrafo(String titulo, String parrafo) {
		super(titulo);
		this.parrafo = parrafo;
		this.t = new Translator();
	}
	
	public Elemento traducir() {
		String parrafoTraducido = t.translate(this.parrafo);
		String tituloTraducido = t.translate(getTitulo());
		return new Parrafo(tituloTraducido, parrafoTraducido);
	}
	
	public String toString() {
		return super.toString() + this.parrafo + "\n";
	}
	
	@Override
	public boolean buscar(String texto) {
		return super.buscar(texto) || this.parrafo.matches(texto);
	}
}
