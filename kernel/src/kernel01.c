#include <stdint.h>
#include <stddef.h>

// Simple i-j-k triple-loop DGEMM implementation
// Signature matches the Rust loader: dgemm(...)
// layout: 101 = RowMajor, 102 = ColMajor
// transa/transb currently only supports 0 (no transpose)

void dgemm(int layout, int transa, int transb, int m, int n, int k,
           double alpha, const double *a, int lda, const double *b, int ldb,
           double beta, double *c, int ldc) {
    if (m <= 0 || n <= 0 || k <= 0) return;

    // Only support non-transposed A and B for this simple kernel
    if (transa != 0 || transb != 0) {
        // fallback: do nothing (user should not use transposes for this simple kernel)
        return;
    }

    if (layout == 101) { // RowMajor
        // A: m x k, lda = k
        // B: k x n, ldb = n
        // C: m x n, ldc = n
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                double sum = 0.0;
                const double *a_row = a + (size_t)i * (size_t)lda;
                const double *b_col = b + (size_t)j; // stride ldb
                for (int p = 0; p < k; ++p) {
                    double av = a_row[p];
                    double bv = b_col[(size_t)p * (size_t)ldb];
                    sum += av * bv;
                }
                double *c_ij = c + (size_t)i * (size_t)ldc + j;
                *c_ij = alpha * sum + beta * (*c_ij);
            }
        }
    } else if (layout == 102) { // ColMajor
        // A: m x k, column-major with lda = m
        // B: k x n, column-major with ldb = k
        // C: m x n, column-major with ldc = m
        for (int j = 0; j < n; ++j) {
            for (int i = 0; i < m; ++i) {
                double sum = 0.0;
                for (int p = 0; p < k; ++p) {
                    double av = a[(size_t)p * (size_t)lda + i];
                    double bv = b[(size_t)j * (size_t)ldb + p];
                    sum += av * bv;
                }
                double *c_ji = c + (size_t)j * (size_t)ldc + i;
                *c_ji = alpha * sum + beta * (*c_ji);
            }
        }
    } else {
        // Unknown layout: do nothing
        return;
    }
}
