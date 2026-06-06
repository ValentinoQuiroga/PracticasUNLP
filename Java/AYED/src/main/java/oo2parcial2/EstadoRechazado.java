package oo2parcial2;

public class EstadoRechazado implements EstadoPrestamo{

	@Override
	public void pagarCuota() {
		throw new Error("error");
	}
	
	@Override
	public double getMontoPagado() {
		return 0;
	}
	
	public double getMontoRestante() {
		throw new Error("error");
	}

	public double getGastosDeCancelacion() {
		throw new Error("error");
	}
}
