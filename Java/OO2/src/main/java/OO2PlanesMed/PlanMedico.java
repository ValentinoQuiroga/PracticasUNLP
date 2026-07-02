package OO2PlanesMed;

public abstract class PlanMedico {
	private double montoFijo;
	
	public PlanMedico(double montoFijo) {
		this.montoFijo = montoFijo;
	}
	
	public double getMontoCobrar(Afiliado afiliado) {
		return this.getMontoFijo(afiliado) + this.getMontoGrupo(afiliado) + this.getMontoViaje(afiliado) + this.getMontoSeguro(afiliado);
	}

	public double getMontoFijo(Afiliado a) {
		return this.montoFijo;
	}
	public abstract double getMontoGrupo(Afiliado a);
	public abstract double getMontoViaje(Afiliado a);
	public abstract double getMontoSeguro(Afiliado a);
}
