package tp2e1;

public class RedBinariaLlena {

	private BinaryTree<Integer> arbol;
	
	public RedBinariaLlena(BinaryTree<Integer> arbol) {
		this.arbol = arbol;
	}
	public int retardoReenvio() {
		return calculo(this.arbol);
	}
	
	private int calculo(BinaryTree<Integer> a) {
		if (a == null) {
			return 0;
		} 
		int izq = (a.getData() + calculo(a.getLeftChild()));
		int der = (a.getData() + calculo(a.getRightChild()));
		
		if (izq >= der) {
			return izq;
		}else {
			return der;
		}
		
	}
	
}
