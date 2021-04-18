CC= clang
CFLAGS=-g -lpthread
BINS=prethread-FTPserver
OBJS=prethread-FTPserver.o micola.o

all: $(BINS)

prethread-FTPserver: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $^

clean:
	rm -rf *.dSYM $(BINS)