# DGEMM Test 프로젝트 아키텍처

다음 문서는 프로젝트의 동작 구조(컴포넌트와 실행 흐름), 빌드 규칙, 그리고 플러그형 커널(.so) 인터페이스를 설명한다.

## 개요
- `dgemm_test` (Rust 바이너리): 사용자 인자 파싱 → `.so` 라이브러리 로드 → `dgemm` 심볼 호출로 벤치마크 수행 → 결과 출력
- `kernel/` 폴더: 커널 소스(`kernel/src/`)와 빌드 산출물(`kernel/lib/`)을 둔다. 각 커널은 공유 라이브러리(`kernel01-knl.so` 등)이다.

## 주요 파일/모듈
- `src/main.rs`: 인자 파싱 및 실행 진입점
- `src/argument.rs`: CLI 옵션 파싱
- `src/exec.rs`: 라이브러리 로드(`libloading`), `dgemm` 심볼 바인딩, 입력 배열 생성, 검증, 반복 실행
- `src/result.rs`: 성능 수치 계산 및 출력
- `kernel/src/kernel01.c`: (예제) i-j-k triple-loop DGEMM 구현
- `kernel/src/kernel02.c`: (예제) i-k-j triple-loop DGEMM 구현
- `Makefile` (프로젝트 루트): `make knl`, `make skl`, `make tx2` 등 CPU별 빌드 타깃
- `kernel/Makefile`: 커널 소스 → `kernel/lib/` 내 여러 `.so` 빌드 규칙 (suffix 지원)

## 플러그 인터페이스 (C 함수 규격)
- 함수명: `dgemm`
- 시그니처 (C):

```c
void dgemm(int layout, int transa, int transb, int m, int n, int k,
           double alpha, const double *a, int lda, const double *b, int ldb,
           double beta, double *c, int ldc);
```

- `layout`: 101 = RowMajor, 102 = ColMajor (프로젝트의 `exec.rs`와 일치)
- `transa`, `transb`: 0 = no-transpose, 1 = transpose (현재 기본 예제는 `0`만 보장함)
- `lda`, `ldb`, `ldc`: 각 배열의 선행 차원(leading dimension)

## 실행 흐름 (시퀀스)

1. 사용자: `cargo run -- ./kernel/lib/kernel01.so --m=128 --n=128 --k=128 --iter=5`
2. `main` → `argument::parse_arguments`로 설정 파싱
3. `exec::execute` 호출
   - 라이브러리 로드 (`libloading::Library::new(lib_path)`) — lib 경로는 첫 번째 인자
   - `dgemm` 심볼 검색 (`lib.get(b"dgemm")`)
   - 입력 벡터/행렬 `A`, `B`, `C` 초기화
   - (옵션) `verify_dgemm`로 정확성 검사
   - warmup 반복 수행
   - 측정 반복 수행(시간 측정) → 결과 수집
4. `result::print`에서 결과 출력 (sec, gflops 등)

## 빌드 규칙(요약)
- 프로젝트 루트에서:

```bash
# No default build: specify a target. Example targets:

# Build for Knights Landing (KNL)
make knl

# Skylake (AVX-512)
make skl

# ThunderX2 CN9980
make tx2
```

- `kernel/Makefile`은 `CC`, `CFLAGS`, `KERNEL_SUFFIX`를 외부에서 전달받아 공유 라이브러리들을 `kernel/lib/`에 생성한다. 예: `KERNEL_SUFFIX=-knl`이면 `kernel01-knl.so`가 생성된다.

## 간단한 다이어그램

- User -> dgemm_test: run with lib path
- dgemm_test -> libloading: open .so
- dgemm_test -> dgemm: call with (layout, m, n, k, pointers)
- dgemm -> dgemm_test: return
- dgemm_test -> stdout: print benchmark

## 예제 명령

```bash
# Build for KNL
make knl

# Run benchmark with KNL-optimized kernel
cargo run -- ./kernel/lib/kernel01-knl.so --m=256 --n=256 --k=256 --iter=3
```