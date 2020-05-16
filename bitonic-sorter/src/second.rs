// pub はこの sort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32] について
//   & は値をポインタ経由で借用することを示す
//   mut は値が変更可能であることを示す
//   u32 型は32ビット符号なし整数
//   [u32] 型はu32のスライス（現時点ではスライスは1次元の配列と考えてよい）
// 型パラメータＴにトレイト境界Ord（全順序）を追加する
pub fn sort<T: Ord>(x: &mut [T], up: bool) {
    // 未実装の意味。コンパイルは通るが、実行すると panic する
    // unimplemented!();

    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[mid_point + i] < x[i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}


// このモジュールは cargo test を実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    // 親モジュール（second）のsort関数を使用する
    use super::sort;

    // #[test] のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_32_ascending() {
        // テストデータとして u32型のベクタを作成しxに束縛する
        // sort関数によって内容が更新されるので、可変を表す mut キーワードが必要
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは &mut x[..]と書いてもいい
        sort(&mut x, true);

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        sort(&mut x, true);
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        // let mut x = vec!["Rust", "is", "a", "system", "programming", "language", "that", "runs"];
        sort(&mut x, false);
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    // #[test]
    // fn sort_f64() {
    //     let mut x = vec![20.0, -30.0, 11.0, 10.0];
    //     sort(&mut x, false);
    // }
    // error[E0277]: the trait bound `{float}: std::cmp::Ord` is not satisfied
}
