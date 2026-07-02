package OO2PlanesMed;

public class PlanPremium extends PlanMedico{

	public PlanPremium(double monto) {
		super(monto);
	}
	
	@Override
	public double getMontoFijo(Afiliado a) {
		double monto = super.getMontoFijo(a);
		if (a.tieneCoseguro()) {
			monto -= a.getDescuento();
		}
		return monto;
	}
	
	public double getMontoGrupo(Afiliado a) {
		if (a.getCantFamiliares() <= 4) {
			return 0;
		}else {
			return a.getCantFamiliares() * 2800;
		}
	}
	
	public double getMontoViaje(Afiliado a) {
		double monto = 0.01 * a.getSalario();
		if (a.tieneCoseguro()) {
			monto -= a.getCoberturaViaje();
		}
		return monto;
	}
	public double getMontoSeguro(Afiliado a) {
		return 0.05 * a.getSalario();
	}
}
