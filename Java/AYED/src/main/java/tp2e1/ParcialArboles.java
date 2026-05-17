package tp2e1;

public class ParcialArboles {
	BinaryTree<Integer> arbol;
	
	public ParcialArboles(BinaryTree<Integer> arbol) {
		this.arbol = arbol;
	}
	
	public boolean isLeftTree(int num) {
		return buscar(arbol, num);
	}
	
	private boolean buscar(BinaryTree<Integer> a, int num) {
		if (a == null) {
			return false;
		}
		if (a.getData() == num) {
			return cumple(a);
		}else {
			return (buscar(a.getLeftChild(), num) | buscar(a.getRightChild(), num));
		}
	}
	
	private boolean cumple(BinaryTree<Integer> a) {
		return(contar(a.getLeftChild()) > contar(a.getRightChild()));
	}
	
	private int contar(BinaryTree<Integer> a) {
		if (a == null) {
			return -1;
		}
		if(a.isLeaf()) {
			return 0;
		}
		if (a.hasLeftChild() && !a.hasRightChild()) {
			return (1 + contar(a.getLeftChild()));
		}else if(!a.hasLeftChild() && a.hasRightChild()){
			return (1 + contar(a.getRightChild()));
		}else {
			return (contar(a.getLeftChild()) + contar(a.getRightChild()));
		}
		
	}
	
	public boolean esPrefijo(BinaryTree<Integer> base, BinaryTree<Integer> arbol) {
		
		if (arbol == null && base == null) {
			return true;
		}else if (arbol == null) {
			return true;
		}else if (base == null){
			return false;
		}
		
		if (arbol.getData() != base.getData()) {
			return false;
		}
		
		return (esPrefijo(base.getLeftChild(), arbol.getLeftChild()) && 
				esPrefijo(base.getRightChild(), arbol.getRightChild()) && true);
	}
}
