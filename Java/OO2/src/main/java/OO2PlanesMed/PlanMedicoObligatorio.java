package OO2PlanesMed;

public class PlanMedicoObligatorio extends PlanMedico{

	public PlanMedicoObligatorio(double monto) {
		super(monto);
	}
	
	public double getMontoGrupo(Afiliado a) {
		double monto = 3500 * a.getCantFamiliares();
		if (a.tieneCoseguro()) { 
			monto = (a.getDescuento() / 100) * monto; 
		}
		return monto;
	}
	
	public double getMontoViaje(Afiliado a) {
		double monto = 0.01 * a.getSalario();
		if (a.tieneCoseguro()) {
			monto -= a.getCoberturaViaje();
		}
		return monto;
	}
	public double getMontoSeguro(Afiliado a) {
		return 0;
	}
}
