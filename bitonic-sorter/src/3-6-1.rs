// use std::{thread, time};

// fn main() {
//     let n1 = 1200;
//     let n2 = 1000;

//     // spawn で子スレッドを立ち上げ、子スレッドで重い処理を実行する
//     // 変数 child がスレッドへのハンドルに束縛される
//     let child = thread::spawn(move || {
//         // 重い処理を実行する
//         // ここに
//         heavy_calc("child", n2)
//     });

//     // 親スレッドでも重い処理を実行する。子スレッドの処理と同時に実行される
//     let s1 = heavy_calc("main", n1);

//     // スレッドのハンドルに対して join を呼ぶことでスレッドの終了を待つ
//     // クロージャの戻り値はOkでラップされる。もしスレッドがエラーにより
//     // 異常終了したら Err が返る
//     match child.join() {
//         Ok(s2) => println!("{}, {}", s1, s2),
//         Err(e) => println!("err: {:?}", e),
//     }
// }

// fn heavy_calc(name: &str, n: u64) -> u64 {
//     println!("{}: started,", name);

//     // 重い処理の代用として nミリ秒スリープする
//     thread::sleep(time::Duration::from_millis(n));

//     // 1からnまでの数字を足し合わせる
//     let sum = (1..=n).sum();
//     println!("{}: ended.", name);
//     sum
// }
