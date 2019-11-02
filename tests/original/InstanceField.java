public class InstanceField {
  public static void main(String[] args){
    Field fa = new Field(1);
    fa.xxxx = 12;
    System.out.println(fa.xxxx);

    Field fb = new Field(2);
    fb.xxxx = 13;
    System.out.println(fb.xxxx);
  }
}

class Field {
  int xxxx;
  public Field(int a) {
    xxxx = a;
    System.out.println(xxxx);
  }
}