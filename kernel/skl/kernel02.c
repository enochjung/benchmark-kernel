#include <stdint.h>

#define CblasRowMajor 101

void cblas_dgemm(int layout, int transa, int transb, int m, int n, int k,
                 double alpha, const double* A, int lda, const double* B,
                 int ldb, double beta, double* C, int ldc) {
    // assumption:
    // - layout == CblasRowMajor
    // - transa == CblasNoTrans
    // - transb == CblasNoTrans
    // - alpha == 1.0
    // - beta == 1.0

    for (int i = 0; i < m; ++i) {
        const double* AA = A + i * lda;
        double* CC = C + i * ldc;

        for (int p = 0; p < k; ++p) {
            const double* a = AA + p;
            const double* BB = B + p * ldb;

            for (int j = 0; j < n; ++j) {
                double* c = CC + j;
                c[0] = a[0] * BB[j] + c[0];
            }
        }
    }
}