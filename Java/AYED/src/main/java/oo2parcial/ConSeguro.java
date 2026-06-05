package oo2parcial;

public class ConSeguro extends DecoratorPaquete{
	
	public ConSeguro(ComponentePaquete paquete) {
		super(paquete);
	}
	
	public double getCostoEnvio() {
		return super.getCostoEnvio() + (super.getValorDeclarado() * 0.20);
	}
	
	public String getDescripcion() {
		return super.getDescripcion() + " con seguro";
	}
}
