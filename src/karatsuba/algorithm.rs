//! Implementation of Karatsuba algorithm to multiply large numbers.

use std::cmp::max;

/// Run the algorithm from input strings.
pub fn run(num1: &str, num2: &str) -> String {
  let x = to_digits(num1);
  let y = to_digits(num2);

  let result_digits = karatsuba(&x, &y);
  digits_to_string(&result_digits)
}

/// Karatsuba multiplication algorithm
fn karatsuba(x: &[u8], y: &[u8]) -> Vec<u8> {
  let n = max(x.len(), y.len());

  // Base case: single-digit multiplication
  if n <= 1 {
    let val_x = if x.is_empty() { 0 } else { x[0] as u64 };
    let val_y = if y.is_empty() { 0 } else { y[0] as u64 };
    let product = val_x * val_y;

    let mut res = Vec::new();
    let mut temp = product;
    if temp == 0 {
      return vec![0];
    }
    while temp > 0 {
      res.push((temp % 10) as u8);
      temp /= 10;
    }
    return res;
  }

  let m = n / 2;

  // Split x and y into high and low parts
  let (x_low, x_high) = if x.len() > m { (&x[..m], &x[m..]) } else { (x, &[][..]) };

  let (y_low, y_high) = if y.len() > m { (&y[..m], &y[m..]) } else { (y, &[][..]) };

  // Recursive steps
  let z0 = karatsuba(x_low, y_low);
  let z2 = karatsuba(x_high, y_high);

  let x_sum = add(x_low, x_high);
  let y_sum = add(y_low, y_high);
  let z1_temp = karatsuba(&x_sum, &y_sum);

  let z1_sub = sub(&z1_temp, &z2);
  let z1 = sub(&z1_sub, &z0);

  // Combine results: z2 * 10^(2*m) + z1 * 10^m + z0
  let term1 = shift_left(&z2, 2 * m);
  let term2 = shift_left(&z1, m);

  let intermediate = add(&term1, &term2);
  add(&intermediate, &z0)
}

/// Helper: Add two vectors of digits (least significant digit first).
fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
  let mut result = Vec::new();
  let mut carry = 0;
  let max_len = max(a.len(), b.len());

  for i in 0..max_len {
    let sum = carry + if i < a.len() { a[i] } else { 0 } + if i < b.len() { b[i] } else { 0 };
    result.push(sum % 10);
    carry = sum / 10;
  }

  if carry > 0 {
    result.push(carry);
  }
  result
}

/// Helper: Subtract b from a (assumes a >= b).
fn sub(a: &[u8], b: &[u8]) -> Vec<u8> {
  let mut result = Vec::new();
  let mut borrow = 0;

  for i in 0..a.len() {
    let b_val = if i < b.len() { b[i] } else { 0 };
    let mut diff = a[i] as i32 - b_val as i32 - borrow;
    if diff < 0 {
      diff += 10;
      borrow = 1;
    } else {
      borrow = 0;
    }
    result.push(diff as u8);
  }

  // Trim trailing zeros
  while result.len() > 1 && *result.last().unwrap() == 0 {
    result.pop();
  }
  result
}

/// Helper: Shift digits left by `n` positions (multiply by 10^n).
fn shift_left(digits: &[u8], n: usize) -> Vec<u8> {
  if digits.len() == 1 && digits[0] == 0 {
    return vec![0];
  }
  let mut result = vec![0; n];
  result.extend_from_slice(digits);
  result
}

/// Helper to convert string/number to reversed digit vector
fn to_digits(s: &str) -> Vec<u8> {
  s.chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

/// Helper to convert digit vector back to string
fn digits_to_string(digits: &[u8]) -> String {
  digits
    .iter()
    .rev()
    .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
    .collect()
}
