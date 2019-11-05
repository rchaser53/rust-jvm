public class NestFor {
  public static void main(String[] args){
    int i = 5;
    int j = 6;
    int count = 0;
    NestForElement[] foo[] = new NestForElement[i][j];
    for (int ii=0; ii<i; ii++) {
        NestForElement bar[] = new NestForElement[j];
        for (int jj=0; jj<j; jj++) {
          bar[jj] = new NestForElement(count);
          count += 1;
        }
        foo[ii] = bar;
    }

    System.out.println(foo[0][1].a);
    System.out.println(foo[3][2].a);
    System.out.println(foo[4][5].a);
  }
}

class NestForElement {
  int a;
  NestForElement(int input) {
    a = input;
  }
}