package ayedParcial;

import tp5.ejercicio1.*;
import tp5.ejercicio1.listaAdy.*;
import ayedParcial.ParcialGrafos;
import java.util.*;

public class ParcialTesteo {

	public static void main(String[] args) {
		Graph<String> red = new AdjListGraph<String>();
		Vertex<String> alpha = red.createVertex("Alpha");
		Vertex<String> beta = red.createVertex("Beta");
		Vertex<String> gamma = red.createVertex("Gamma");
		Vertex<String> delta = red.createVertex("Delta");
		Vertex<String> epsilon = red.createVertex("Epsilon");
		Vertex<String> servidor_final = red.createVertex("ServidorFinal");

		red.connect(alpha, beta, 4);
		red.connect(alpha, gamma, 7);
		red.connect(beta, gamma, 6);
		red.connect(beta, delta, 3);
		red.connect(gamma, delta, 9);
		red.connect(gamma, epsilon, 4);
		red.connect(delta, epsilon, 2);
		red.connect(delta, servidor_final, 7);
		red.connect(epsilon, servidor_final, 5);
		
		ParcialGrafos p = new ParcialGrafos();
		List<String> l = p.rutaConCifrado(red, "Alpha", "ServidorFinal");
		for (String a: l) {
			System.out.println(a);
		}
	}

}
