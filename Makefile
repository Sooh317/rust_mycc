# CFLAGS=-std=c11 -g -static

mycc: 
	/home/user/.cargo/bin/cargo build

test:
	docker run --rm -v ~/Desktop/Compiler/mycc:/9cc -w /9cc compilerbook make test1

test1: mycc
	./test.sh

clean:
	rm -f *.o *~ tmp*

.PHONY: test clean