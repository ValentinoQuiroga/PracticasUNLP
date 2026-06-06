package oo2parcial2;

public class EstadoAprobado implements EstadoPrestamo {
    private int cuotasRestantes;
    private Prestamo contexto;
    
    public EstadoAprobado(int cuotasRestantes, Prestamo contexto) {
        this.cuotasRestantes = cuotasRestantes;
        this.contexto = contexto;
    }
    
    @Override
    public void pagarCuota() {
        contexto.registrarPago(); // 1. Suma el dinero de forma segura al acumulador
        cuotasRestantes -= 1;     // 2. Descuenta la cuota restante
        
        if (cuotasRestantes == 0) {
            contexto.setEstado(new EstadoFinalizado(contexto)); // 3. Cambia de estado si terminó
        }
    }
    
    @Override
    public double getMontoPagado() {
        return contexto.getMontoAcumulado();
    }
    
    @Override
    public double getMontoRestante() {
        return contexto.determinarValorDeCuota() * cuotasRestantes;
    }
    
    @Override
    public double getGastosDeCancelacion() {
        return (getMontoRestante() * 1.1) + contexto.getGastosSellado();
    }
}
