use divan;

fn main() {
  divan::main();
}

const SIZE: usize = 10;
const fn sizes() -> [u64; SIZE] {
  let mut arr: [u64; SIZE] = [0; SIZE];
  let mut i = 0;
  while i < SIZE {
    arr[i] = 5u64.pow((i + 1) as u32);
    i += 1;
  }
  arr
}

#[divan::bench(consts = sizes(), skip_ext_time = false)]
fn bench<const N: u64>(bencher: divan::Bencher) {
  bencher
    .with_inputs(|| gap_1::powersort::generator::generate_random_vector(N))
    .bench_values(|input| {
      gap_1::powersort::algorithm::powersort(divan::black_box(input.clone()));
    });
}
