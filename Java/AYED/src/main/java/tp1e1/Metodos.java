package tp1e1;

public class Metodos {

	int a;
	int b;
	public static void main(String[] args) {
		metodoFor(4, 9);
		metodoWhile(5,15);
		metodoSinEstructura(1,7);
		
	}
	
	private static void metodoFor(int a, int b) {
		for (int i = a; i <= b; i++) {
			System.out.println("f: "+ i);
		}
	}
	
	private static void metodoWhile(int a, int b) {
		int i = a;
		while (i != b) {
			System.out.println("w: " + i);
			i++;
		}
	}

	private static void metodoSinEstructura(int a, int b) {
		System.out.println("sin: " + a);
		if (a != b) {
			metodoSinEstructura(a+1,b);
		}
	}
}
