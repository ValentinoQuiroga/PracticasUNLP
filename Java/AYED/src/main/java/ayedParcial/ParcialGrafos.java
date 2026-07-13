package ayedParcial;
import java.util.*;
import tp5.ejercicio1.*;

public class ParcialGrafos {
	public ParcialGrafos(){}
	public List<String> rutaConCifrado(Graph<String> red, String origen, String destino){
		List<String> lista = new ArrayList<String>();
		List<Vertex<String>> vertices = red.getVertices();
		Iterator<Vertex<String>> iter = vertices.iterator();
		Vertex<String> entrada = null;
		Vertex<String> salida = null;
		Vertex<String> aux = null;
		while ((iter.hasNext()) && ((entrada == null) || (salida == null))){
			aux = iter.next();
			if (aux.getData().equals(origen)) {
				entrada = aux;
			}
			if (aux.getData().equals(destino)) {
				salida = aux;
			}
		}
		
		if ((entrada != null) && (salida != null)) {
			boolean[] marcas = new boolean[vertices.size()];
			realizarBusqueda(red, entrada, salida, true, marcas, lista);
		}
		return lista;
	}
	private boolean realizarBusqueda(Graph<String> red, Vertex<String> origen, Vertex<String> destino,
			boolean par, boolean[] marcas, List<String> lista) {
		if (origen.getData().equals(destino.getData())){
			lista.add(origen.getData());
			return true;
		}
		List<Edge<String>> vecinos = red.getEdges(origen);
		Iterator<Edge<String>> iter_v = vecinos.iterator();
		Edge<String> aux = null;
		marcas[origen.getPosition()] = true;
		boolean encontrado = false;
		
		while ((!encontrado) && (iter_v.hasNext())){
			aux = iter_v.next();
			if (!marcas[aux.getTarget().getPosition()]) {
				if (par) {
					if (aux.getWeight() % 2 == 0) {
						encontrado = realizarBusqueda(red, aux.getTarget(), destino, false, marcas, lista);
					}
				}else {
					if (aux.getWeight() % 2 == 1) {
						encontrado = realizarBusqueda(red, aux.getTarget(), destino, true, marcas, lista);
					}
					
				}
			}
		}
		if (encontrado) {
			lista.addFirst(origen.getData());
		}
		return encontrado;
	}
}
