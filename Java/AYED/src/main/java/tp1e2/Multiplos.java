package tp1e2;
import java.util.Scanner;
public class Multiplos {

	public static void main(String[] args) {
			System.out.println("Ingrese un numero: ");
			Scanner s = new Scanner (System.in);
			int n = s.nextInt();
			int [] arreglo = new int[n];
			for (int i = 0; i < n; i++) {
				arreglo[i] = n * (i + 1);
			}
			
			for (int i = 0; i < n; i++) {
				System.out.println(arreglo[i]);
			}
	}
}
