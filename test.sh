#!/bin/bash
assert(){
    expected="$1"
    input="$2"
    
    ./target/debug/mycc "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    ret="$?"

    if [ "$ret" = "$expected" ]; then
        echo "$input => $ret"
    else
        echo "$input => $expected expected, but got $ret"
        exit 1
    fi
}

assert_func(){
    input="$1"

    ./target/debug/mycc "$input" > tmp.s
    cc -static -o tmp tmp.s func.o
    ./tmp
}

assert 0 'int main(){0;}'
assert 42 'int main(){42;}'
assert 21 'int main(){5+20-4;}'
assert 41 'int main(){ 12 + 34 - 5 ;}'
assert 47 'int main(){5+6*7;}'
assert 15 'int main(){5*(9-6);}'
assert 4 'int main(){(3+5)/2;}'
assert 10 'int main(){-10+20;}'
assert 109 'int main(){-2*-(43)+23;}'
assert 2 'int main(){3*(-(-2))*-3+20;}'
assert 16 'int main(){-2 + -3*-4  * 3/-2/-1;}'
assert 0 'int main(){3 == 4;}'
assert 1 'int main(){-2*-3 <= 1+2+3;}'
assert 0 'int main(){-2*-3 < 1+2+3;}'
assert 1 'int main(){-2*-3 >= 1+2+3;}'
assert 0 'int main(){-2*-3 > (1+2+3)/3*8;}'
assert 1 'int main(){2 == 1+4/4+1-1 == 3 < 4 == 8 >= 7;}'
assert 1 'int main(){1<= 8 != 3 > 2*4;}'
assert 2 'int main(){int a; a = 2;}'
assert 14 'int main(){int a; int b; a = 3; b = 5 * 6 - 8; a + b / 2;}'
assert 1 'int main(){int a; int b; int c; a = 0; b = a == 0; c = a + b + 2; int d; d = c > 2;}'
assert 6 'int main(){int foo; foo = 1; int bar; bar = 2 + 3; foo + bar;}'
assert 4 'int main(){int a; int aa; int foo; int hoge; a = 2; aa = 4; foo = a * aa - aa; hoge = foo / 2 + a;}'
assert 4 'int main(){return 4;}'
assert 4 'int main(){int a; a = 5; return a - 1;}'
assert 3 'int main(){return 3 + 4 / 2 * 2 - 3 + (-(9)) + -2 + 10;}'
assert 4 'int main(){int a; int foo; a = 2; int aa; aa = 4; foo = a * aa - aa; return foo;}'
assert 1 'int main(){if (1 == 1) return 1; else return 2;}'
assert 4 'int main(){int a; a = 5; if (a == 5) a = a - 1; return a;}'
assert 4 'int main(){if (2 >= 4 - 3*2 + 2) return 4; else return 100;}'
assert 3 'int main(){if (2 < 4 - 3*2 + 2) return 4; else if(2 == 3) return 1; else return 3 + 4 / 2 * 2 - 3 + (-(9)) + -2 + 10;}'
assert 4 'int main(){int a; a = 5; while(a != 4) a = a - 1; return a;}'
assert 16 'int main(){int a; int b; a = 4; b = 0; while(b < 15) b = b + a; return b;}' 
assert 10 'int main(){int sum; sum = 0; for(int i = 0; i <= 4; i = i + 1) sum = sum + i; return sum;}'
assert 5 'int main(){int rem; rem = 4; int tmp; tmp = 0; for(; rem >= 0; rem = rem - 1) tmp = tmp + 1; return tmp;}'
assert 2 'int main(){int a; a = 2; for(;;) return a;}'
assert 5 'int main(){{int a; int b; a = 1; b = a + 2; return b + 2;}}'
assert 6 'int main(){int sum; sum = 0; for(int i = 1; i <= 4; i = i + 1){sum = sum + i; sum = sum - 1;} return sum;}'
assert 12 'int main(){int a; a = 2*3; if(a == 6){int b; b = 0; while(b < 3){a = a + 2; b = b + 1;} return a;}}'
assert_func 'int main(){for(int i = 0; i <= 1; i = i + 1){foo();}}'
assert_func 'int main(){foo2(3, 4);}'
assert_func 'int main(){int a; int b; int c; a = 5; b = 6; c = a * b; foo3(a, b, c);}'
assert_func 'int main(){int a; a = 5; int b; b = 6; foo3(a, a * b, - a * b);}'
assert_func 'int main(){int a; a = 5; int b; b = 6; foo4(a + b, a - b, a * b, - a * b);}'
assert_func 'int main(){int c; int a; a = 5; int b; b = 6; c = 100; foo5(a + b, c + -c + c + -c + c, a * b, b * c, c * a);}'
assert 55 'int fib(int a){ if(a <= 1){return a;} else{return fib(a - 1) + fib(a - 2);}} int main(){return fib(10);}'
assert 3 'int main(){ int x; x = 3; int *y; y = &x; return *y;}'
assert 5 'int main(){ int x; x = 3; int y; y = 5; int* z = &y; return *z;}'
assert 3 'int main(){ int x; x=3; return *&x; }'
assert 3 'int main(){ int x; x=3; int* y; y=&x; int** z; z=&y; return **z; }'
assert 5 'int main(){ int x; int *y; x=3; y=&x; *y=5; return x; }'
assert 5 'int main(){ int x; x=3; int y; y=5; return *(&x-1); }'
assert 3 'int main(){ int x; x=3; int y; y=5; return *(&y+1); }'
assert 7 'int main(){ int x; x=3; int y; y=5; *(&x-1)=7; return y; }'
assert 7 'int main(){ int x; x=3; int y; y=5; *(&y+1)=7; return x; }'
assert 6 'int main(){ int x; x=3; int* y; y = &x; int** z; z = &y; int*** w; w = &z; return ***w + 3;}'
assert 4 'int main(){ int x; x=3; return sizeof(x); }'
assert 4 'int main(){ int x; x=3; return sizeof(sizeof(x)); }'
assert 8 'int main(){ int* x; return sizeof(x); }'
assert 12 'int main(){ int* x; return sizeof(x) + sizeof(sizeof x); }'
assert 4 'int main(){return sizeof(2);}'
assert 4 'int main(){ int* x; return sizeof(x - x);}'
echo OK