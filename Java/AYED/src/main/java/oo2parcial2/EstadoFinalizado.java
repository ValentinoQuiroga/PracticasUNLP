package oo2parcial2;

public class EstadoFinalizado implements EstadoPrestamo{
	private Prestamo contexto;
	
	public EstadoFinalizado(Prestamo contexto) {
		this.contexto = contexto;
	}
	public void pagarCuota() {
		throw new Error("Prestamo saldado");
	}
	
	public double getMontoPagado() {
		return (contexto.getMontoAcumulado());
	}
	public double getMontoRestante() {
		return 0.00;
	}

	public double getGastosDeCancelacion() {
		return 0;
	}
}
