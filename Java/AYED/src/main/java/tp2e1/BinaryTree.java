package tp2e1;



public class BinaryTree <T> {
	
	private T data;
	private BinaryTree<T> leftChild;   
	private BinaryTree<T> rightChild; 

	
	public BinaryTree() {
		super();
	}

	public BinaryTree(T data) {
		this.data = data;
	}

	public T getData() {
		return data;
	}

	public void setData(T data) {
		this.data = data;
	}
	/**
	 * Preguntar antes de invocar si hasLeftChild()
	 * @return
	 */
	public BinaryTree<T> getLeftChild() {
		return leftChild;
	}
	/**
	 * Preguntar antes de invocar si hasRightChild()
	 * @return
	 */
	public BinaryTree<T> getRightChild() {
		return this.rightChild;
	}

	public void addLeftChild(BinaryTree<T> child) {
		this.leftChild = child;
	}

	public void addRightChild(BinaryTree<T> child) {
		this.rightChild = child;
	}

	public void removeLeftChild() {
		this.leftChild = null;
	}

	public void removeRightChild() {
		this.rightChild = null;
	}

	public boolean isEmpty(){
		return (this.isLeaf() && this.getData() == null);
	}

	public boolean isLeaf() {
		return (!this.hasLeftChild() && !this.hasRightChild());

	}
		
	public boolean hasLeftChild() {
		return this.leftChild!=null;
	}

	public boolean hasRightChild() {
		return this.rightChild!=null;
	}
	@Override
	public String toString() {
		return this.getData().toString();
	}

	public  int contarHojas() {
		return contar(this);
	}
	
	private int contar(BinaryTree b) {
		if (b == null) {
			return 0;
		}
		if (b.isLeaf()) {
			return 1;
		}else {
			return contar(b.getLeftChild()) + contar(b.getRightChild());
		}
	}
		
		
    	 
    public BinaryTree<T> espejo(){
		BinaryTree<T> esp = new BinaryTree<T>();
		return espejear(this);
    }
    
    private BinaryTree espejear(BinaryTree o){;
    	if (o == null) {
    		return null;
    	}
    	BinaryTree a = new BinaryTree(o.getData());
    	a.addLeftChild(o.getRightChild());
    	a.addRightChild(o.getLeftChild());
    	espejear(o.getLeftChild());
   		espejear(o.getRightChild());
   		
   		return a;
    	} 
    

	// 0<=n<=m
	public void entreNiveles(int n, int m){
		am(this, 0, n, m);
	}
	
	private void am(BinaryTree a, int i, int n, int m) {
		if (a != null) {
			if ((i >= n) && (i <= m)) {
				System.out.println(a.getData());
			}
			am(a.getLeftChild(), i + 1, n ,m);
			am(a.getRightChild(), i + 1, n ,m);
		}
	}
	
	
	public void imprimir() {
		imp(this);
	}
	
	private void imp(BinaryTree a) {
		if (a != null) {
			System.out.println(a.getData());
			imp(a.getLeftChild());
			imp(a.getRightChild());
		}
	}
		
}

