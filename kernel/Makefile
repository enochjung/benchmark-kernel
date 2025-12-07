LDFLAGS := -shared -O3 -fopenmp -lm -Wall -Werror

.PHONY: knl skl tx2 clean

knl:
	@for file in knl/*.c; do \
		gcc -O3 -march=knl -mtune=knl -fPIC $(LDFLAGS) -o $${file%.c}.so $$file; \
	done

skl:
	@for file in skl/*.c; do \
		gcc -O3 -march=skylake-avx512 -mtune=skylake-avx512 -fPIC $(LDFLAGS) -o $${file%.c}.so $$file; \
	done

tx2:
	@for file in tx2/*.c; do \
		aarch64-linux-gnu-gcc -O3 -mcpu=thunderx2t99 -fPIC $(LDFLAGS) -o $${file%.c}.so $$file; \
	done

clean:
	rm -f knl/*.so skl/*.so tx2/*.so
