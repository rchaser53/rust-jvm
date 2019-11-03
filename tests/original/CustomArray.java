public class CustomArray {
  public static void main(String[] args){
    ArrayElementClass foo[] = new ArrayElementClass[3];
    foo[0] = new ArrayElementClass(1);
    System.out.println(foo[0].a);
  }
}

class ArrayElementClass {
  int a;
  ArrayElementClass(int input) {
    a = input;
  }
}