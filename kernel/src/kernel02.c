#include <stdint.h>
#include <stddef.h>

// DGEMM implementation with loop order i-k-j
// Signature matches the Rust loader: dgemm(...)
// layout: 101 = RowMajor, 102 = ColMajor
// transa/transb currently only supports 0 (no transpose)

void dgemm(int layout, int transa, int transb, int m, int n, int k,
           double alpha, const double *a, int lda, const double *b, int ldb,
           double beta, double *c, int ldc) {
    if (m <= 0 || n <= 0 || k <= 0) return;

    if (transa != 0 || transb != 0) return;

    if (layout == 101) { // RowMajor
        // A: m x k, lda = k
        // B: k x n, ldb = n
        // C: m x n, ldc = n
        for (int i = 0; i < m; ++i) {
            const double *a_row = a + (size_t)i * (size_t)lda;
            double *c_row = c + (size_t)i * (size_t)ldc;
            for (int p = 0; p < k; ++p) {
                double av = a_row[p];
                const double *b_row = b + (size_t)p * (size_t)ldb;
                for (int j = 0; j < n; ++j) {
                    c_row[j] = alpha * av * b_row[j] + beta * c_row[j];
                }
            }
        }
    } else if (layout == 102) { // ColMajor
        for (int j = 0; j < n; ++j) {
            for (int p = 0; p < k; ++p) {
                double bv = b[(size_t)j * (size_t)ldb + p];
                for (int i = 0; i < m; ++i) {
                    double av = a[(size_t)p * (size_t)lda + i];
                    double *c_ji = c + (size_t)j * (size_t)ldc + i;
                    *c_ji = alpha * av * bv + beta * (*c_ji);
                }
            }
        }
    }
}
