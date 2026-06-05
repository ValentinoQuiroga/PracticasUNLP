package oo2parcial;

public class DecoratorPaquete implements ComponentePaquete{

	private ComponentePaquete paquete;

	public DecoratorPaquete(ComponentePaquete paquete) {
		this.paquete = paquete;
	}
	@Override
	public String getDescripcion() {return paquete.getDescripcion();
	}

	@Override
	public double getCostoEnvio() {return paquete.getCostoEnvio();
	}

	@Override
	public double getValorDeclarado() {return paquete.getValorDeclarado();
	}
	
	
	
	
}
