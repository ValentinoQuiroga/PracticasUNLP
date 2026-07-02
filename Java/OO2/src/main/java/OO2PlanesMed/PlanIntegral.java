package OO2PlanesMed;

public class PlanIntegral extends PlanMedico{

	public PlanIntegral(double monto) {
		super(monto);
	}
	
	public double getMontoGrupo(Afiliado a) {
		return (3000 * a.getCantFamiliares()) + (0.01 * a.getSalario());
	}
	
	public double getMontoViaje(Afiliado a) {
		double monto = 0.03 * a.getSalario();
		if (a.tieneCoseguro()) {
			monto -= (a.getAntiguedad() * 10000);
		}
		return monto;
	}
	public double getMontoSeguro(Afiliado a) {
		return 0.05 * a.getSalario();
	}
}
