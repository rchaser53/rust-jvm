public class CustomMultiDimentionArray {
  public static void main(String[] args){
    ArrayElementClass[] foo[] = new ArrayElementClass[3][4];
    foo[1][2] = new ArrayElementClass(11);
    System.out.println(foo[1][2].a);
  }
}

class ArrayElementClass {
  int a;
  ArrayElementClass(int input) {
    a = input;
  }
}