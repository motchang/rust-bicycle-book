// pub はこの sort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32] について
//   & は値をポインタ経由で借用することを示す
//   mut は値が変更可能であることを示す
//   u32 型は32ビット符号なし整数
//   [u32] 型はu32のスライス（現時点ではスライスは1次元の配列と考えてよい）
pub fn sort(x: &mut [u32], up: bool) {
    // 未実装の意味。コンパイルは通るが、実行すると panic する
    // unimplemented!();

    if x.len() > 1 {
	let mid_point = x.len() / 2;
	sort(&mut x[..mid_point], true);
	sort(&mut x[mid_point..], false);
	sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
