// pub はこの sort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32] について
//   & は値をポインタ経由で借用することを示す
//   mut は値が変更可能であることを示す
//   u32 型は32ビット符号なし整数
//   [u32] 型はu32のスライス（現時点ではスライスは1次元の配列と考えてよい）
// 型パラメータＴにトレイト境界Ord（全順序）を追加する
use super::SortOrder;
use rayon;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sort を呼ぶ代わりに、sort_by を呼ぶようにする
    // is_power_of_two は sort_by が呼ぶので、ここからは削除した
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    // 未実装の意味。コンパイルは通るが、実行すると panic する
    // unimplemented!();

    if x.len() > 1 {
        let mid_point = x.len() / 2;

        // これは lef first = の行で xに対する可変の参照が作られ、それが返却される前に
        // let second = の行で再度可変の借用を作ろうとしているためエラーとなる
        // let first = &mut x[..mid_point];
        // let second = &mut x[mid_point..];
        let (first, second) = x.split_at_mut(mid_point);

        // xの分割後の要素数を閾値 PARALLEL_THRESHOLD と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // 閾値以上なら並列にソートする（並列処理）

            // comparator クロージャの型Fには Sync 境界が必要で、xの要素の型TにはSend境界が必要

            // error[E0277]: `F` cannot be shared between threads safely
            // error[E0277]: `T` cannot be sent between threads safely

            rayon::join(
                || do_sort(first, true, comparator),
                || do_sort(second, false, comparator),
            );
        } else {
            // x をバイトニックソートする
            // 第二引数が true のときは comparator で示される順序でソート
            do_sort(first, true, comparator);
            // 第二引数が false のときは comparator とは逆順でソート
            do_sort(second, false, comparator);
        }

        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);

        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(
                || sub_sort(first, forward, comparator),
                || sub_sort(second, forward, comparator),
            );
        } else {
            sub_sort(first, forward, comparator);
            sub_sort(second, forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // 比較に先立ち forward(bool) を Orderingに変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    let mid_point = x.len() / 2;

    for i in 0..mid_point {
        // comparator クロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_condition とひとしいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i)
        }
    }
}

// このモジュールは cargo test を実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    use super::{sort, sort_by};
    use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
    use crate::SortOrder::*;

    // impl PartialEq for Student {
    //     fn eq(&self, other: &Self) -> bool{
    //         // selfとotherですべてのフィールド同士を比較して、どのフィールドも等しいなら
    //         // selfとotherは等しい
    //         self.first_name == other.first_name
    //             && self.second_name == other.second_name
    //             && self.age == other.age
    //     }
    // }
    // Partialeq トレイトと Debug トレイトは自動導出というコードの自動生成に対応しているので
    // そちらを使おう
    #[derive(Debug, PartialEq)]
    struct Student {
        // 構造体 Student を定義する
        // 構造体は関連する値を一つにまとめたデータ構造。複数のデータフィールドを持つ。
        // 構造体の定義ではフィールドの型を省略できない
        first_name: String,  // first_name(名前) フィールド。
        second_name: String, // second_name(苗字) フィールド。
        age: u8,             // 年齢
    }

    // impl ブロックを使うと、対象の型に関連関数やメソッドを実装できる
    impl Student {
        // new 関数は関連関数にあたり、Student::new(...)の形式で呼び出します
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体 Student を初期化して返す。Selfは impl 対象の型（Student）の別名
            Self {
                // to_string メソッドで &str 型の引数から String 型の値を作る。
                first_name: first_name.to_string(), // first_name フィールドに値を設定
                second_name: last_name.to_string(), // last_name フィールドに値を設定
                age,                                // ageフィールドにage変数の値を設定
                                                    // フィールドと変数が同じ名前のときは、このように省略形で書ける
            }
        }
    }

    #[test]
    // 年齢で昇順にソートする
    fn sort_student_by_age_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // sort_by 関数でソートする。第二引数はソート順を決めるクロージャ
            // 引数にふたつのStudent構造体をとり、ageフィールドの値を cmp メソッドで比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, &|a, b| a
                .second_name
                .cmp(&b.second_name)
                // もし last_name が等しくない（LessまはたGreater）ならをれを返す
                // last_name が等しい（Equal）ならfirst_nameを比較する
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );
        assert_eq!(x, expected);
    }

    // #[test] のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_32_ascending() {
        // テストデータとして u32型のベクタを作成しxに束縛する
        // sort関数によって内容が更新されるので、可変を表す mut キーワードが必要
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは &mut x[..]と書いてもいい
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        // let mut x = vec!["Rust", "is", "a", "system", "programming", "language", "that", "runs"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err())
    }

    // #[test]
    // fn sort_f64() {
    //     let mut x = vec![20.0, -30.0, 11.0, 10.0];
    //     sort(&mut x, false);
    // }
    // error[E0277]: the trait bound `{float}: std::cmp::Ord` is not satisfied

    #[test]
    fn sort_u32_large() {
        {
            // 乱数で 65,536要素のデータ列を作る(65536は2の16乗)
            let mut x = new_u32_vec(65536);
            // 昇順にソートする
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // ソート結果が正しいことを検証する
            assert!(is_sorted_ascending(&x));
        };
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        };
    }
}
