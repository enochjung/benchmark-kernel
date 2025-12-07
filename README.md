dgemm_test
===========

DGEMM benchmark tool that loads optimized kernel implementations from shared libraries.

## Quick Start

```bash
# Build for target architecture
make knl      # Knights Landing
make skl      # Skylake (AVX-512)
make tx2      # ThunderX2 CN9980

# Run benchmark
./target/release/dgemm_test <kernel-dir> [options..]

# Benchmark example
./target/release/dgemm_test ./kernel/lib/kernel01-knl.so --m=128 --n=128 --k=128 --iter=3
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

## Notes

- Kernels are in `kernel/src/`, compiled to `kernel/lib/` with `.so` suffix