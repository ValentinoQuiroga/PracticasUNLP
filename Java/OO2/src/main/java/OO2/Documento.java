package OO2;

public class Documento {
	private String titulo;
	private String autor;
	private Elemento seccionRaiz;
	private Translator t;
	
	public Documento(String titulo, String autor, Elemento elemento) {
		this.titulo = titulo;
		this.autor = autor;
		this.seccionRaiz = elemento;
		this.t = new Translator();
	}
	
	public String toString() {
		return  this.titulo + " - " + this.autor + "\n" + seccionRaiz.toString();
	}
	
	public boolean buscar(String texto) {
		return seccionRaiz.buscar(texto);
	}
	
	public Documento traducir(){
		String tituloTraducido = t.translate(titulo);
		String autorTraducido = t.translate(autor);
		Elemento seccionTraducida = seccionRaiz.traducir();
		return new Documento(tituloTraducido, autorTraducido, seccionTraducida);
	}
}
