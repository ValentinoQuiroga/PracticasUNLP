package OO2;
import OO2PlanesMed.*;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class AfiliadoTest {
	Afiliado a;
	Coseguro c;
	
	@BeforeEach
	void setUp() {
		PlanMedico p = new PlanMedicoObligatorio(15000);
		a = new Afiliado(null, 2, 0, 100000, 0, p);
	}
	
	@Test
	void test() {
		assertEquals(a.calcularMonto(), 23000);
	}
}
