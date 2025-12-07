LDFLAGS := -shared
KERNEL_SRC_DIR := src
KERNEL_LIB_DIR := lib

SRCS := $(wildcard $(KERNEL_SRC_DIR)/*.c)
BASENAMES := $(patsubst $(KERNEL_SRC_DIR)/%.c,%,$(SRCS))
TARGETS := $(patsubst %, $(KERNEL_LIB_DIR)/%$(KERNEL_SUFFIX).so,$(BASENAMES))

# Check required variables only when building (not for clean)
ifeq ($(MAKECMDGOALS),)
ifeq ($(MAKECMDGOALS),all)
ifndef CC
$(error CC is not defined. Please specify CC when invoking make)
endif
ifndef CFLAGS
$(error CFLAGS is not defined. Please specify CFLAGS when invoking make)
endif
ifndef KERNEL_SUFFIX
$(error KERNEL_SUFFIX is not defined. Please specify KERNEL_SUFFIX when invoking make)
endif
endif
endif

ifeq ($(filter clean,$(MAKECMDGOALS)),)
ifndef CC
$(error CC is not defined. Please specify CC when invoking make)
endif
ifndef CFLAGS
$(error CFLAGS is not defined. Please specify CFLAGS when invoking make)
endif
ifndef KERNEL_SUFFIX
$(error KERNEL_SUFFIX is not defined. Please specify KERNEL_SUFFIX when invoking make)
endif
endif

all: $(TARGETS)

$(KERNEL_LIB_DIR):
	mkdir -p $(KERNEL_LIB_DIR)

$(KERNEL_LIB_DIR)/%$(KERNEL_SUFFIX).so: $(KERNEL_SRC_DIR)/%.c | $(KERNEL_LIB_DIR)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

clean:
	rm -f $(KERNEL_LIB_DIR)/*.so

.PHONY: all clean
