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

assert 0 'main(){0;}'
assert 42 'main(){42;}'
assert 21 "main(){5+20-4;}"
assert 41 "main(){ 12 + 34 - 5 ;}"
assert 47 'main(){5+6*7;}'
assert 15 'main(){5*(9-6);}'
assert 4 'main(){(3+5)/2;}'
assert 10 'main(){-10+20;}'
assert 109 'main(){-2*-(43)+23;}'
assert 2 'main(){3*(-(-2))*-3+20;}'
assert 16 'main(){-2 + -3*-4  * 3/-2/-1;}'
assert 0 'main(){3 == 4;}'
assert 1 'main(){-2*-3 <= 1+2+3;}'
assert 0 'main(){-2*-3 < 1+2+3;}'
assert 1 'main(){-2*-3 >= 1+2+3;}'
assert 0 'main(){-2*-3 > (1+2+3)/3*8;}'
assert 1 'main(){2 == 1+4/4+1-1 == 3 < 4 == 8 >= 7;}'
assert 1 'main(){1<= 8 != 3 > 2*4;}'
assert 2 'main(){a = 2;}'
assert 14 'main(){a = 3; b = 5 * 6 - 8;  a + b / 2;}'
assert 1 'main(){a = 0; b = a == 0; c = a + b + 2; d = c > 2;}'
assert 6 'main(){foo = 1; bar = 2 + 3; foo + bar;}'
assert 4 'main(){a = 2; aa = 4; foo = a * aa - aa; hoge = foo / 2 + a;}'
assert 4 'main(){return 4;}'
assert 4 'main(){a = 5; return a - 1;}'
assert 3 'main(){return 3 + 4 / 2 * 2 - 3 + (-(9)) + -2 + 10;}'
assert 4 'main(){a = 2; aa = 4; foo = a * aa - aa; return foo;}'
assert 1 'main(){if (1 == 1) return 1; else return 2;}'
assert 4 'main(){a = 5; if (a == 5) a = a - 1; return a;}'
assert 4 'main(){if (2 >= 4 - 3*2 + 2) return 4; else return 100;}'
assert 3 'main(){if (2 < 4 - 3*2 + 2) return 4; else if(2 == 3) return 1; else return 3 + 4 / 2 * 2 - 3 + (-(9)) + -2 + 10;}'
assert 4 'main(){a = 5; while(a != 4) a = a - 1; return a;}'
assert 16 'main(){a = 4; b = 0; while(b < 15) b = b + a; return b;}' 
assert 10 'main(){sum = 0; for(i = 0; i <= 4; i = i + 1) sum = sum + i; return sum;}'
assert 5 'main(){rem = 4; tmp = 0; for(; rem >= 0; rem = rem - 1) tmp = tmp + 1; return tmp;}'
assert 2 'main(){a = 2; for(;;) return a;}'
assert 5 'main(){{a = 1; b = a + 2; return b + 2;}}'
assert 6 'main(){sum = 0; for(i = 1; i <= 4; i = i + 1){sum = sum + i; sum = sum - 1;} return sum;}'
assert 12 'main(){a = 2*3; if(a == 6){ b = 0; while(b < 3){a = a + 2; b = b + 1;} return a;}}'
assert_func 'main(){for(i = 0; i <= 1; i = i + 1){foo();}}'
assert_func 'main(){foo2(3, 4);}'
assert_func 'main(){a = 5; b = 6; c = a * b; foo3(a, b, c);}'
assert_func 'main(){a = 5; b = 6; foo3(a, a * b, - a * b);}'
assert_func 'main(){a = 5; b = 6; foo4(a + b, a - b, a * b, - a * b);}'
assert_func 'main(){a = 5; b = 6; c = 100; foo5(a + b, c + -c + c + -c + c, a * b, b * c, c * a);}'
assert 55 'fib(a){ if(a <= 1){return a;} else{return fib(a - 1) + fib(a - 2);}} main(){return fib(10);}'
assert 3 'main(){x = 3; y = &x; return *y;}'
assert 3 'main(){ x = 3; y = 5; z = &y + 8; return *z;}'
echo OK