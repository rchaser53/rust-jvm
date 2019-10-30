public class InitializeStatic {
  static int sa = 3;
  public static void main(String[] args){
    System.out.println(InitializeStatic.sa + OtherClass.sb);
    InitializeStatic.sa = 5;
    OtherClass.sb += 1;
    System.out.println(InitializeStatic.sa + OtherClass.sb);
  }
}

class OtherClass {
  static int sb = 4;
}