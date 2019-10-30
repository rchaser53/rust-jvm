public class NewAndCallInstanceMethod {
  public static void main(String[] args){
    CallInstanceMethod a = new CallInstanceMethod();
    a.abc();
    System.out.println(">>> created");
  }
}

class CallInstanceMethod {
  CallInstanceMethod() {
    System.out.println(">>> constructor");
  }

  void abc() {
    System.out.println(">>> abc");
  }
}