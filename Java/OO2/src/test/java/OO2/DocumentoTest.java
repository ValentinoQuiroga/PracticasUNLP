package OO2;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;


public class DocumentoTest {
	Documento doc;
	
	@BeforeEach
	void setUp() {
		Seccion raiz = new Seccion("Seccion A");
		doc = new Documento("", "", raiz);
	}
}
