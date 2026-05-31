CC     := gcc
CFLAGS := -shared -O3 -fPIC -fopenmp -lm -Wall -Werror

.PHONY: knl skl tx2 clean

knl:
	@for file in knl/*.c; do \
		$(CC) -o $${file%.c}.so $$file -march=knl -mtune=knl -lnuma $(CFLAGS); \
	done

skl:
	@for file in skl/*.c; do \
		$(CC) -o $${file%.c}.so $$file -march=skylake-avx512 -mtune=skylake-avx512 $(CFLAGS); \
	done

tx2:
	@for file in tx2/*.c; do \
		$(CC) -o $${file%.c}.so $$file -mcpu=thunderx2t99 $(CFLAGS); \
	done

clean:
	rm -f knl/*.so skl/*.so tx2/*.so
