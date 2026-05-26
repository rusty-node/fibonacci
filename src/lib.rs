#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Maximum `n` for which Fibonacci(n) fits in `i64`.
const MAX_I64_N: u32 = 92;

#[napi]
pub fn fibonacci(n: u32) -> Result<i64> {
  if n > MAX_I64_N {
    return Err(Error::from_reason(format!(
      "n must be <= {MAX_I64_N} (Fibonacci(93) overflows i64)"
    )));
  }
  Ok(fib_i64(n))
}

fn fib_i64(n: u32) -> i64 {
  if n == 0 {
    return 0;
  }
  let (mut a, mut b) = (0_i64, 1_i64);
  for _ in 1..n {
    let next = a + b;
    a = b;
    b = next;
  }
  b
}
