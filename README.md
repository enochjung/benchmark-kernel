dgemm_test
===========

DGEMM benchmark tool that loads optimized kernel implementations from shared libraries.

## Quick Start

```bash
# Build for target architecture
make knl      # Intel Xeon Phi 7250 (KNL)
make skl      # Intel Xeon 6148 (Skylake)
make tx2      # Marvell ThunderX2 CN9980

# Run benchmark
./target/release/dgemm_test <kernel.so> [options..]

# Benchmark example
./target/release/dgemm_test ./kernel/knl/kernel01.so --m=128 --n=128 --k=128 --iter=3
```

## Options

```
--nthreads=N      Thread count
--layout=row|col  Matrix layout (default: row)
--transa=T|F      Transpose A (default: F)
--transb=T|F      Transpose B (default: F)
--m=N, --n=N, --k=N    Matrix dimensions
--alpha=DOUBLE    Alpha parameter
--beta=DOUBLE     Beta parameter
--warmup=N        Warmup iterations
--iter=N          Benchmark iterations
--verify          Verify correctness
--result-policy=min|avg|max
--only-result     Output only result
```