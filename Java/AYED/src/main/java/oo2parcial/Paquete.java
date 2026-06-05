package oo2parcial;

public class Paquete implements ComponentePaquete{
	private String descripcion;
	private String destinatario;
	private String direccionDestino;
	private double valorDeclarado;
	
	public Paquete( String descripcion, String destinatario, String direccionDestino, double valorDeclarado) {
		this.descripcion = descripcion;
		this.destinatario = destinatario;
		this.direccionDestino = direccionDestino;
		this.valorDeclarado = valorDeclarado;
	}

	@Override
	public String getDescripcion() {return this.descripcion;}
	public String getDestinatario() {return this.destinatario;}
	public String getDireccionDestino() {return this.direccionDestino;}

	@Override
	public double getValorDeclarado() {return this.valorDeclarado;}

	@Override
	public double getCostoEnvio() {return this.valorDeclarado * 0.05;}
}
