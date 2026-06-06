package oo2parcial2;

public class EstrategiaPrestamoUVA implements EstrategiaPrestamo {
    private double tasaDeInteres; // Cambiado de int a double

    public EstrategiaPrestamoUVA(double tasaDeInteres) {
        this.tasaDeInteres = tasaDeInteres;
    }
    
    @Override
    public double determinarValorDeCuota(double monto, int cuotas) {
        // Obviamos la clase Indec externa para que compile de forma autónoma
        return (monto / cuotas) * (1 + tasaDeInteres);
    }

    @Override
    public double getGastosSellado() {
        return 0; // Bonificado
    }
}
