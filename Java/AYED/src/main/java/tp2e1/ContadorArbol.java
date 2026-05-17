package tp2e1;
import java.util.*;
public class ContadorArbol {
	BinaryTree<Integer> arbol = new BinaryTree();
	
	public ContadorArbol(BinaryTree<Integer> arbol) {
		this.arbol = arbol;
	}
	
	public List<Integer> numerosPares(){
		List<Integer> lista = new ArrayList<>();
		recorrido(arbol, lista);
		return lista;
	}
	
	private void recorrido(BinaryTree<Integer> a, List<Integer> l) {
		if (a == null) {return;}
		
		recorrido(a.getLeftChild(),l);
		
		if( (a.getData() % 2) == 0) {
			l.add(a.getData());
		}
		
		recorrido(a.getRightChild(),l);
	}
}
