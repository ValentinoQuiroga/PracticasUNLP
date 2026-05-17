package tp1e5;

public class Resultado {
	private int max;
	private int min;
	private int pro;
	
	public Resultado(int x, int n, int o) {
		this.max = x;
		this.min = n;
		this.pro = o;
	}
	
	public int getMax() {
		return max;
	}
	public void setMax(int max) {
		this.max = max;
	}
	public int getMin() {
		return min;
	}
	public void setMin(int min) {
		this.min = min;
	}
	public int getPro() {
		return pro;
	}
	public void setPro(int pro) {
		this.pro = pro;
	}
	
	
}
