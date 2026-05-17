package tp1e7;
import java.util.*;

public class TestArrayList {
	
	public static void main(String[] args){
		/*ArrayList<Integer> array = new ArrayList<>(Arrays.asList(1,2,3,4,5));
		LinkedList<Integer> arreglo = new LinkedList<>(Arrays.asList(6,7,8,9));

		for (Integer i: array){
			arreglo.add(i);
		}
		for (Integer i: arreglo) {
			System.out.println(i);
		}
		*/
		//metodo();
		ArrayList<Integer> c = calcular(7);
		invertirArrayList(c);
		for (int n: c) {
			System.out.println("c:" + n);
			System.out.println("tam c: " + c.size());
		}
		LinkedList<Integer> l = convertir(c);
		System.out.println(c.isEmpty());
		System.out.println(sumarLinkedList(l));
		System.out.println(l.isEmpty());
		
	}
	
	public static LinkedList<Integer> convertir(ArrayList<Integer> lista){
		LinkedList<Integer> linked = new LinkedList<>();
		for (Integer i: lista) {
			linked.add(i);
			System.out.println("v: " + i);
		}
		return linked;
	}
	
	public static int sumarLinkedList(LinkedList<Integer> lista) {
		if (lista.isEmpty()) {
			System.out.println("vacio");
			return 0;
		}else {
			int n = lista.getLast();
			lista.removeLast();
			System.out.println("valor" + n);
			return (n + sumarLinkedList(lista));
		}
	}
	
	public static ArrayList<Integer> calcular(int n){
		ArrayList<Integer> s = new ArrayList<>();
		cal(n, s);
		return s;
	}
	
	private static void cal(int n, ArrayList<Integer> s) {
		s.add(n);
		
		if (n == 1) {
			return;
		}
		if (n % 2 == 0) {
			cal(n / 2, s);
		}else {
			cal((3 * n) + 1, s);
		}
		
	}
	
	public static void invertirArrayList (ArrayList<Integer> s) {
		ArrayList<Integer> t = new ArrayList<>();
		invertir(s,t);
		s=t;
	}
	
	private static void invertir(ArrayList<Integer> s, ArrayList<Integer> t){
		if (s.size() != 0) {
			t.add(s.getLast());
			s.removeLast();
			System.out.println(t.getLast());
			invertir(s,t);
		}else {
			s = t;
			System.out.println(s.size());
		}
	}
	
	public static void metodo() {
		ArrayList<String> estudiantesA = new ArrayList<>(Arrays.asList("a","b","c"));
		ArrayList<String> estudiantesB = new ArrayList<>(estudiantesA);
		
		for (String a: estudiantesA) {
			System.out.println(a);
		}
		for (String b: estudiantesB) {
			System.out.println(b);
		}

		estudiantesA.set(1, "Beee");
		estudiantesB.set(1, "Emmm");
		
		for (String a: estudiantesA) {
			System.out.println(a);
		}
		for (String b: estudiantesB) {
			System.out.println(b);
		}
	}
}
