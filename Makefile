CC := gcc
KERNEL_DIR := kernel

.PHONY: all
all:
	$(error Please specify a build target: knl, skl, or tx2)

knl:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) CFLAGS="-O3 -march=knl -mtune=knl -fPIC" KERNEL_SUFFIX="-knl"

skl:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) CFLAGS="-O3 -march=skylake-avx512 -mtune=skylake-avx512 -fPIC" KERNEL_SUFFIX="-skl"

tx2:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) CFLAGS="-O3 -march=native -mtune=native -fPIC" KERNEL_SUFFIX="-tx2"

.PHONY: clean
clean:
	$(MAKE) -C $(KERNEL_DIR) clean