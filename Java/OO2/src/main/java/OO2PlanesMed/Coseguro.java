package OO2PlanesMed;

public class Coseguro {
	private int descuento;
	private double cobertura;
	
	public Coseguro(int descuento, double cobertura) {
		this.descuento = descuento;
		this.cobertura = cobertura;
	}
	public int getDescuento() {return this.descuento;}
	public double getMontoCoberturaViajes() {return this.cobertura;}
}
