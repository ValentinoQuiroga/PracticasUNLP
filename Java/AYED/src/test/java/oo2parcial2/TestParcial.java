package oo2parcial2;

import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class TestParcial {

	Prestamo pres;
	Cliente cli;
	Prestamo pres2;
	Cliente cli2;
	@BeforeEach
	void setUp(){
		cli = new Cliente("",5000);
		cli2 = new Cliente("", 1);
		pres = new Prestamo(100, 10, cli, new EstrategiaPrestamoSimple(0.05));
		pres2 = new Prestamo(100, 10, cli2, new EstrategiaPrestamoSimple(0.05));
	}

	@Test
	void test() {
		assertEquals(10.5, pres.determinarValorDeCuota());
	}
	@Test
	void test2() {
		assertEquals(0, pres.getMontoPagado());
	}
	@Test
	void test3() {
		pres.pagarCuota();
		pres.pagarCuota();
		assertEquals(21.0, pres.getMontoPagado());
	}
	@Test
	void test4() {
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		pres.pagarCuota();
		assertEquals(0, pres.getGastosDeCancelacion());
		assertEquals(0, pres.getMontoRestante());
	}
	@Test
	void test5() {
		assertThrows(Error.class, () -> {
	        pres2.pagarCuota(); // <-- El código que debería explotar
	    });
	}

}
