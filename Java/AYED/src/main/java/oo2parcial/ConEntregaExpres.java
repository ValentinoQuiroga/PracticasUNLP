package oo2parcial;

public class ConEntregaExpres extends DecoratorPaquete{
	
	public ConEntregaExpres(ComponentePaquete paquete) {super(paquete);}
	
	public double getCostoEnvio() {
		return super.getCostoEnvio() + ( super.getValorDeclarado() * 0.5);
	}
	
	public String getDescripcion() {
		return super.getDescripcion() + " entrega express";
	}
}
