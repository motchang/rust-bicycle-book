use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNG を初期化する。再現性を持たせるため、毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // n個の要素が格納できるようベクタを初期化する
    let mut v = Vec::with_capacity(n);

    // 0 から n-1 までの合計 n回、繰り返し乱数を生成し、ベクタに追加する
    // (0から n-1の数列は使わないので、_で受け取ることで、すぐに破棄している)
    for _ in 0..n {
        // RNG の sample メソッドは引数として与えられた分布にしたがう乱数を1つ生成する
        // Standard分布は生成する値が整数型（ここではu32型）の時は一様分布
        // になる。つまり、その型が取りうるすべての値が同じ確率で出現する
        v.push(rng.sample(&Standard));
    }

    v
}
