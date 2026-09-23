//! Implementation of Powersort algorithm to sort vectors.

const MIN_RUN: usize = 32;

#[derive(Debug, Clone, Copy)]
struct Run {
  start: usize,
  len: usize,
  power: u32,
}

/// Computes the node power (priority) of a run boundary using bitwise operations.
fn node_power(n: usize, i1: usize, n1: usize, i2: usize, n2: usize) -> u32 {
  if n <= 1 {
    return 0;
  }
  let num1 = (2 * i1 + n1) as u128;
  let num2 = (2 * i2 + n2) as u128;
  let den = (2 * n) as u128;

  let left = (num1 << 64) / den;
  let right = (num2 << 64) / den;

  let diff = left ^ right;
  diff.leading_zeros()
}

/// Binary insertion sort for extending short runs.
fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
  let n = arr.len();
  for i in 1..n {
    let x = arr[i].clone();
    let mut left = 0;
    let mut right = i;
    while left < right {
      let mid = left + (right - left) / 2;
      if x < arr[mid] {
        right = mid;
      } else {
        left = mid + 1;
      }
    }
    for j in (left..i).rev() {
      arr[j + 1] = arr[j].clone();
    }
    arr[left] = x;
  }
}

/// Stable merge of two adjacent sorted slices within the vector using a temporary buffer.
fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
  let mut temp = Vec::with_capacity(arr.len());
  temp.extend_from_slice(arr);

  let mut i = 0;
  let mut j = mid;
  let n = arr.len();
  let mut k = 0;

  while i < mid && j < n {
    if temp[i] <= temp[j] {
      arr[k] = temp[i].clone();
      i += 1;
    } else {
      arr[k] = temp[j].clone();
      j += 1;
    }
    k += 1;
  }

  while i < mid {
    arr[k] = temp[i].clone();
    i += 1;
    k += 1;
  }

  while j < n {
    arr[k] = temp[j].clone();
    j += 1;
    k += 1;
  }
}

/// Main Powersort entry point taking and returning an owned Vec<T>.
pub fn powersort<T: Ord + Clone>(mut arr: Vec<T>) -> Vec<T> {
  let n = arr.len();
  if n <= 1 {
    return arr;
  }

  let mut runs: Vec<Run> = Vec::new();
  let mut i = 0;

  // Helper closure to discover and extend natural runs
  let next_run = |arr: &mut [T], start: usize, n_total: usize| -> (usize, usize) {
    if start >= n_total {
      return (start, 0);
    }
    let mut end = start + 1;
    if end < n_total {
      if arr[end] < arr[end - 1] {
        // Descending run: reverse it to make it ascending (maintaining stability)
        while end < n_total && arr[end] < arr[end - 1] {
          end += 1;
        }
        arr[start..end].reverse();
      } else {
        // Ascending run
        while end < n_total && arr[end] >= arr[end - 1] {
          end += 1;
        }
      }
    } else {
      end = n_total;
    }

    let mut len = end - start;
    if len < MIN_RUN && end < n_total {
      let target_end = std::cmp::min(start + MIN_RUN, n_total);
      binary_insertion_sort(&mut arr[start..target_end]);
      end = target_end;
      len = end - start;
    } else if len < MIN_RUN {
      binary_insertion_sort(&mut arr[start..end]);
    }

    (start, len)
  };

  // Find the first run
  let (s1, l1) = next_run(&mut arr, i, n);
  if l1 == 0 {
    return arr;
  }
  runs.push(Run {
    start: s1,
    len: l1,
    power: 0,
  });
  i = s1 + l1;

  // Scan the rest of the array and maintain the stack invariant
  while i < n {
    let (s2, l2) = next_run(&mut arr, i, n);
    if l2 == 0 {
      break;
    }
    let r1 = runs.last().unwrap();
    let p = node_power(n, r1.start, r1.len, s2, l2);

    while p <= runs.last().unwrap().power {
      let r2_run = runs.pop().unwrap();
      let r1_run = runs.pop().unwrap();

      let merged_start = r1_run.start;
      let merged_len = r1_run.len + r2_run.len;
      let slice_end = r2_run.start + r2_run.len;

      merge(&mut arr[merged_start..slice_end], r1_run.len);

      runs.push(Run {
        start: merged_start,
        len: merged_len,
        power: r1_run.power,
      });
    }

    runs.push(Run {
      start: s2,
      len: l2,
      power: p,
    });
    i = s2 + l2;
  }

  // Flush remaining runs on the stack right-to-left
  while runs.len() > 1 {
    let r2_run = runs.pop().unwrap();
    let r1_run = runs.pop().unwrap();

    let merged_start = r1_run.start;
    let merged_len = r1_run.len + r2_run.len;
    let slice_end = r2_run.start + r2_run.len;

    merge(&mut arr[merged_start..slice_end], r1_run.len);

    runs.push(Run {
      start: merged_start,
      len: merged_len,
      power: 0,
    });
  }

  arr
}
