package oo2parcial2;

public class EstrategiaPrestamoSimple implements EstrategiaPrestamo {
    private double tasaDeInteres; // Cambiado de int a double

    public EstrategiaPrestamoSimple(double tasaDeInteres) {
        this.tasaDeInteres = tasaDeInteres;
    }
    
    @Override
    public double determinarValorDeCuota(double monto, int cuotas) {
        return (monto / cuotas) * (1 + tasaDeInteres);
    }
    
    @Override
    public double getGastosSellado() {
        return 5000;
    }
}
