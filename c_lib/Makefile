CC = gcc
CFLAGS = -Wall -Wextra -O2 -fPIC

all: libc_lib.a

libc_lib.a: c_lib.o
	ar rcs $@ $^

c_lib.o: c_lib.c c_lib.h
	$(CC) $(CFLAGS) -c c_lib.c

clean:
	rm -f *.o *.a
