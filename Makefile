CC := gcc
KERNEL_DIR := kernel

.PHONY: all knl skl tx2 clean
all:
	$(error Please specify a build target: knl, skl, or tx2)

knl:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) knl

skl:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) skl

tx2:
	$(MAKE) -C $(KERNEL_DIR) CC=$(CC) tx2

clean:
	$(MAKE) -C $(KERNEL_DIR) clean