package tp3;

import java.util.LinkedList;
import java.util.Queue;

public class RedAguaPotable {
	private GeneralTree<Character> estructura;
	
	public RedAguaPotable(GeneralTree<Character> estructura){
		this.estructura = estructura;
	}
	
	public double minimoCaudal(double caudal) {
		return contarCaudal(estructura, 9999, caudal);
	}
	
	private double contarCaudal(GeneralTree<Character> a, double caudalMin, double caudalActual) {
		if (a == null) {
			return caudalMin;
		}
		if ((a.isLeaf()) && (caudalActual < caudalMin )){
			return caudalActual;
		}
		for (GeneralTree<Character> hijo: a.getChildren()) {
			double valorRecuperado = contarCaudal(hijo, caudalMin, caudalActual / a.getChildren().size());
			if (valorRecuperado < caudalMin) {
				caudalMin = valorRecuperado;
			}
		}
		return caudalMin;
	}
		
}
