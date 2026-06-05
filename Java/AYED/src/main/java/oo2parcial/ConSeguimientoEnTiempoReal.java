package oo2parcial;

public class ConSeguimientoEnTiempoReal extends DecoratorPaquete{

	public ConSeguimientoEnTiempoReal(ComponentePaquete paquete) {super(paquete);}
	
	public double getCostoEnvio() {
		return super.getCostoEnvio() + 2000;
	}
}
