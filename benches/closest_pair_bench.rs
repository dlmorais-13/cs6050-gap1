use divan;

fn main() {
  divan::main();
}

const POW_BASE: u64 = 4;
const SIZE: usize = 10;

const fn sizes() -> [u64; SIZE] {
  let mut arr: [u64; SIZE] = [0; SIZE];
  let mut i = 0;
  while i < SIZE {
    arr[i] = POW_BASE.pow((i + 1) as u32);
    i += 1;
  }
  arr
}

#[divan::bench(consts = sizes())]
fn bench<const N: u64>(bencher: divan::Bencher) {
  bencher
    .with_inputs(|| gap_1::closest_pair::generator::generate_random_vector(N))
    .bench_values(|input| {
      gap_1::closest_pair::algorithm::closest_pair(divan::black_box(input.clone()));
    });
}
