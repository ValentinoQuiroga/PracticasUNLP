package OO2;

public abstract class Texto implements Elemento{
	private String titulo;
	
	public String toString() {
		return this.titulo + "\n";
	}
	
	public abstract Elemento traducir();
	
	public String getTitulo() {
		return this.titulo;
	}
	public Texto(String titulo) {
		this.titulo = titulo;
	}

	public boolean buscar(String texto) {
		return this.titulo.matches(texto);
	}
}
