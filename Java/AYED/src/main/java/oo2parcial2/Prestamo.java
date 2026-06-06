package oo2parcial2;

public class Prestamo { // Se eliminó el "abstract"
    private double monto;
    private double montoAbonado;
    private int cantCuotas;
    private Cliente cliente;
    private EstadoPrestamo estadoActual;
    private EstrategiaPrestamo estrategiaActual;
    
    public Prestamo(double monto, int cantCuotas, Cliente cliente, EstrategiaPrestamo estrategiaActual) {
        this.monto = monto;
        this.cantCuotas = cantCuotas;
        this.cliente = cliente;
        this.estrategiaActual = estrategiaActual;
        this.montoAbonado = 0.0;
        evaluarSalario();
    }
    
    private void evaluarSalario() {
        if (this.determinarValorDeCuota() > (this.cliente.getSalario() * 0.3)) {
            setEstado(new EstadoRechazado());
        } else { 
            setEstado(new EstadoAprobado(cantCuotas, this));
        }
    }
    
    // Nuevo método seguro invocado por el estado para registrar el cobro
    public void registrarPago() {
        this.montoAbonado += determinarValorDeCuota();
    }
    
    public void pagarCuota() {
        estadoActual.pagarCuota(); // Delegación pura
    }
    
    public double getMontoPagado() {
        return estadoActual.getMontoPagado();
    }
    
    public double getMontoAcumulado() {
        return this.montoAbonado;
    }
    
    public double getMontoRestante() {
        return estadoActual.getMontoRestante();
    }
    
    public void setEstado(EstadoPrestamo estadoActual) {
        this.estadoActual = estadoActual;
    }
    
    public int getCantCuotas() {
        return this.cantCuotas;
    }
    
    public double determinarValorDeCuota() {
        return estrategiaActual.determinarValorDeCuota(monto, cantCuotas);
    }
    
    public double getGastosDeCancelacion() {
        return estadoActual.getGastosDeCancelacion();
    }
    
    public double getGastosSellado() {
        return estrategiaActual.getGastosSellado();
    }
}
