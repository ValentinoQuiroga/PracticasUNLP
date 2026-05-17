package tp1e9;
import java.util.*;
public class TestBalanceo {

	public TestBalanceo() {}
	
	public boolean estaBalanceado(String s) {
		List<Character> caracteres = new ArrayList<Character>();
		List<Character> ab = Arrays.asList('[', '{', '(');
		List<Character> ce = Arrays.asList(']', '}', ')');
		
		for (char c: s.toCharArray()) {
			if (ab.contains(c) | ce.contains(c)) {
				caracteres.add(c);
			}
		}
		
		boolean b = true;
		List<Character> stack = new LinkedList<Character>();
		
		for (char c: caracteres) {
			if (ab.contains(c)) {
				stack.addFirst(c);
			}else if (stack.isEmpty()) {
				b = false;
			}else{
				if (c == ')' & stack.getFirst() == '(' ) {
					stack.removeFirst();
				}else if (c == '}' & stack.getFirst() == '{' ) {
					stack.removeFirst();
				}else if (c == ']' & stack.getFirst() == '[' ) {
					stack.removeFirst();
				}else b = false;
			}
		}
		
		
		return b;
	}
}
