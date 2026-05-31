benchmark-kernel
===========

Kernel benchmark tool that loads optimized kernel implementations from shared libraries.

## Quick Start

```bash
# Build benchmark-kernel
cargo build --release

# Build kernel libraries for target architecture
make knl      # Intel Xeon Phi 7250 (KNL)
make skl      # Intel Xeon 6148 (Skylake)
make tx2      # Marvell ThunderX2 CN9980

# Run benchmark
./target/release/bench COMMAND kernel-lib [options...]

# Benchmark example
./target/release/bench dgemm ./kernel/knl/kernel01.so --m=128 --n=128 --k=128 --iter=3
```

## Commands

### `dgemm`

Double-precision general matrix–matrix multiplication (GEMM).
Computes `C` += `A` * `B` where `C` is (`m` x `n`), `A` is (`m` x `k`), and `B` is (`k` x `n`).

```bash
Options:                                               Default:
  --nthreads=INTEGER       Number of threads             1
  --layout=[row|col]       Matrix layout                 row
  --transa=[true|false]    Transpose A                   false
  --transb=[true|false]    Transpose B                   false
  --alpha=DOUBLE           Scalar alpha                  1.0
  --beta=DOUBLE            Scalar beta                   1.0
  --m=INTEGER              Rows of A / C                 100
  --n=INTEGER              Columns of B / C              100
  --k=INTEGER              Inner dimension               100
  --warmup=INTEGER         Warmup iterations             0
  --iter=INTEGER           Benchmark iterations          5
  --verify                 Enable correctness check      (off)
  --summary=[min|avg|max]  Summary aggregation method    max
  --concise                Print a single-line summary   (off)
```

### `spmv`

HPCG-style sparse matrix–vector multiply using ELLPACK with block size 32.
The problem size is $n^3$ (`n` × `n` × `n` blocks).

```bash
Options:                                               Default:
  --n=INTEGER              Block grid size               104
  --warmup=INTEGER         Warmup iterations             0
  --iter=INTEGER           Benchmark iterations          5
  --verify                 Enable correctness check      (off)
  --summary=[min|avg|max]  Summary aggregation method    max
  --concise                Print a single-line summary   (off)
```