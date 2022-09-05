int foo(){ printf("test(foo)\n");}
int foo1(int x){printf("test(foo2) : x = %d\n", x);}
int foo2(int x, int y){printf("test(foo2) : x + y = %d\n", x + y);}
int foo3(int x, int y, int z){printf("test(foo3) : x + y + z = %d + %d + %d\n", x, y, z);}
int foo4(int x, int y, int z, int w){printf("test(foo4) : x + y + z + w = %d + %d + % d + %d\n", x, y, z, w);}
int foo5(int x, int y, int z, int w, int v){printf("test(foo5) : x + y + z + w + v = %d + %d + % d + %d + %d\n", x, y, z, w, v);}
