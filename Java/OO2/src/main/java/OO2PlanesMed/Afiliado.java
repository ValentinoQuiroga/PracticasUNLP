package OO2PlanesMed;

public class Afiliado {
	private Coseguro coseguro;
	private int familiaresACargo;
	private int descuento;
	private double salario;
	private double coberturaViaje;
	private PlanMedico planMedico;
	
	public Afiliado(Coseguro coseguro, int familiaresACargo, int descuento, double salario, double coberturaViaje, PlanMedico planMedico) {
		this.coseguro = coseguro;
		this.familiaresACargo = familiaresACargo;
		this.descuento = descuento;
		this.salario = salario;
		this.coberturaViaje = coberturaViaje;
		this.planMedico = planMedico;
		
	}
	
	public double calcularMonto() {
		return planMedico.getMontoCobrar(this);
	}
	
	public boolean tieneCoseguro() {
		return !(coseguro == null);
	}
	
	public int getCantFamiliares() {
		return this.familiaresACargo;
	}
	
	public int getDescuento() {
		return this.descuento;
	}
	
	public double getSalario() {
		return this.salario;
	}
	
	public double getCoberturaViaje() {
		return this.coberturaViaje;
	}
	
	public double getAntiguedad() {
		return 2;
	}
	
}

	
