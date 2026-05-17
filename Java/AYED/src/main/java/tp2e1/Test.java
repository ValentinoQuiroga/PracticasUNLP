package tp2e1;

public class Test {

	public static void main(String[] args) {
		BinaryTree a = new BinaryTree(2);
		BinaryTree b = new BinaryTree(7);
		BinaryTree c = new BinaryTree(23);
		BinaryTree d = new BinaryTree(-3);
		BinaryTree e = new BinaryTree(6);
		BinaryTree f = new BinaryTree(55);
		BinaryTree g = new BinaryTree(11);
		BinaryTree h = new BinaryTree(-5);
		BinaryTree i = new BinaryTree(19);
		BinaryTree j = new BinaryTree(4);
		BinaryTree k = new BinaryTree(18);

		BinaryTree aa = new BinaryTree(2);
		BinaryTree ba = new BinaryTree(7);
		BinaryTree ca = new BinaryTree(23);
		BinaryTree da = new BinaryTree(-3);
		BinaryTree ea = new BinaryTree(6);
		BinaryTree fa = new BinaryTree(55);/*
		BinaryTree ga = new BinaryTree(11);
		BinaryTree ha = new BinaryTree(-5);
		BinaryTree ia = new BinaryTree(19);
		BinaryTree ja = new BinaryTree(4);
		BinaryTree ka = new BinaryTree(18);*/
		
		a.addLeftChild(b);
		b.addLeftChild(c);
		c.addLeftChild(d);
			b.addRightChild(e);
		e.addLeftChild(f);
			e.addRightChild(g);
			a.addRightChild(h);
		h.addLeftChild(i);
			i.addRightChild(j);
		j.addLeftChild(k);
		
		aa.addLeftChild(ba);
		ba.addLeftChild(ca);
		ca.addLeftChild(da);
			ba.addRightChild(ea);
		ea.addLeftChild(fa);
	
		//System.out.println(a.contarHojas());
		//a.imprimir();
		//BinaryTree espejo = a.espejo();
		//espejo.imprimir();
		//a.entreNiveles(1, 10);
		//ContadorArbol cont = new ContadorArbol(a);
		//for (int i: cont.numerosPares()) {
		//	System.out.println(i);
		//}
		//RedBinariaLlena rbl = new RedBinariaLlena(a);
		//System.out.println(rbl.retardoReenvio());
		//ProfundidadDeArbolBinario pab = new ProfundidadDeArbolBinario(a);
		//System.out.println(pab.sumaElementosProfundidad(2));
		//Transformacion t = new Transformacion(a);
		//t.suma().imprimir();;
		//ParcialArboles p = new ParcialArboles(a);
		ParcialArboles pa = new ParcialArboles(a);
		System.out.println(pa.esPrefijo(a, aa));
	}

}
