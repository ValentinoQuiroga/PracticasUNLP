package tp2e1;

public class ProfundidadDeArbolBinario {
	private BinaryTree<Integer> arbol;
	
	public ProfundidadDeArbolBinario(BinaryTree<Integer> arbol) {
		this.arbol = arbol;
	}
	
	public int sumaElementosProfundidad(int p) {
		int i = 0;
		return recorrido(arbol, i, p);
	}
	private int recorrido(BinaryTree<Integer> a, int i, int p) {
		if (a == null) {
			return 0;
		}
		if (i == p) {
			return a.getData();
		}
		if (i < p) {
			return (recorrido(a.getLeftChild(), i + 1, p) + recorrido(a.getRightChild(), i + 1, p));
		}else {
			return 0;
		}
	}
}
