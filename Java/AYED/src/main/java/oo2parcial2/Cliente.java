package oo2parcial2;

public class Cliente {
	private String nombre;
	private double salario;
	
	public Cliente(String nombre, double salario) {
		this.nombre = nombre;
		this.salario = salario;
	}
	
	public double getSalario() {
		return this.salario;
	}
}
