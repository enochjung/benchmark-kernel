#include <stdint.h>
#include <stddef.h>

void dgemm(int layout, int transa, int transb, int m, int n, int k,
           double alpha, const double *a, int lda, const double *b, int ldb,
           double beta, double *c, int ldc) {
    if (m <= 0 || n <= 0 || k <= 0) return;

    if (transa != 0 || transb != 0) {
        return;
    }

    if (layout == 101) {
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                double sum = 0.0;
                const double *a_row = a + (size_t)i * (size_t)lda;
                const double *b_col = b + (size_t)j;
                for (int p = 0; p < k; ++p) {
                    double av = a_row[p];
                    double bv = b_col[(size_t)p * (size_t)ldb];
                    sum += av * bv;
                }
                double *c_ij = c + (size_t)i * (size_t)ldc + j;
                *c_ij = alpha * sum + beta * (*c_ij);
            }
        }
    } else if (layout == 102) {
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
        return;
    }
}
