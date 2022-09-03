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

assert 0 0 
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34 - 5 "
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 10 '-10+20'
assert 109 '-2*-(43)+23'
assert 2 '3*(-(-2))*-3+20'
assert 16 '-2 + -3*-4  * 3/-2/-1'
assert 0 '3 == 4'
assert 1 '-2*-3 <= 1+2+3'
assert 0 '-2*-3 < 1+2+3'
assert 1 '-2*-3 >= 1+2+3'
assert 0 '-2*-3 > (1+2+3)/3*8'
assert 1 '2 == 1+4/4+1-1 == 3 < 4 == 8 >= 7'
assert 1 '1<= 8 != 3 > 2*4'

echo OK