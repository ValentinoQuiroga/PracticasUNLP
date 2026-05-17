package tp2e1;

public class Transformacion {
	private BinaryTree<Integer> arbol;
	
	public Transformacion(BinaryTree<Integer> arbol) {
		this.arbol = arbol;
	}
	
	public BinaryTree<Integer> suma(){
		BinaryTree<Integer> res = new BinaryTree<Integer>();
		calculo(arbol,res);
		return res;
		
	}
	
	private int calculo(BinaryTree<Integer> a, BinaryTree<Integer> r){
		if (a == null) {
			r = null;
			return 0;
		}
		if (a.isLeaf()) {
			r.setData(0);
			return a.getData();
		}
		if (a.hasLeftChild()) {
			r.addLeftChild(new BinaryTree());
		}
		if (a.hasRightChild()) {
			r.addRightChild(new BinaryTree());
		}
		r.setData(calculo(a.getLeftChild(), r.getLeftChild()) + calculo(a.getRightChild(), r.getRightChild()));
		return r.getData();
	}
}
