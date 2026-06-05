package oo2parcial;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;

import org.junit.jupiter.api.Test;

class TestCajaDeLibros {

	
	Paquete paq;
	DecoratorPaquete paqDecorado;
	
	@BeforeEach
	void setUp() throws Exception {
		paq = new Paquete("Caja de libros","Valen","La Plata",20000);
		paqDecorado = new ConSeguro(paq);
		paqDecorado = new ConEntregaExpres(paqDecorado);
		}
	
    @Test
    public void testNombreCompleto() {
        assertEquals(15000, paqDecorado.getCostoEnvio());
        assertEquals("Caja de libros con seguro entrega express", paqDecorado.getDescripcion());
    }
}