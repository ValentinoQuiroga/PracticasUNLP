package tp1e5;

public class CalculoEnArreglo {

	private int[] arreglo = {1,2,3,4,5,6,7,8,9,10};
	private Resultado res = new Resultado(-9999,9999,0);
	
	public Resultado metodoUno() {
		Resultado r = new Resultado(-9999, 9999, 0);
		int p = 0;
		for (int i = 0; i < arreglo.length; i++) {
			if (r.getMax() < arreglo[i]) {
				r.setMax(arreglo[i]);
			}
			if (r.getMin() > arreglo[i]) {
				r.setMin(arreglo[i]);
			}
			p += arreglo[i];
		}
		p = p / arreglo.length;
		r.setPro(p);
		
		return r;
	}
	
	public void metodoDos() {
		int p = 0;
		for (int i = 0; i < arreglo.length; i++) {
			if (res.getMax() < arreglo[i]) {
				res.setMax(arreglo[i]);
			}
			if (res.getMin() > arreglo[i]) {
				res.setMin(arreglo[i]);
			}
			p += arreglo[i];
		}
		p = p / arreglo.length;
		res.setPro(p);
	}
	
	
}
