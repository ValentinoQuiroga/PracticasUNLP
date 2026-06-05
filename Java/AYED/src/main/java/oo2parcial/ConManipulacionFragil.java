package oo2parcial;

public class ConManipulacionFragil extends DecoratorPaquete{
	
	public ConManipulacionFragil(ComponentePaquete paquete) {super(paquete);}
	
	public String getDescripcion() {
		return super.getDescripcion() + " fragil";
	}
	
}
