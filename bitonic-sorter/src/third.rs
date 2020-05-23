// pub はこの sort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32] について
//   & は値をポインタ経由で借用することを示す
//   mut は値が変更可能であることを示す
//   u32 型は32ビット符号なし整数
//   [u32] 型はu32のスライス（現時点ではスライスは1次元の配列と考えてよい）
// 型パラメータＴにトレイト境界Ord（全順序）を追加する
use super::SortOrder;

pub fn sort<T: Ord>(x: &mut[T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    // 未実装の意味。コンパイルは通るが、実行すると panic する
    // unimplemented!();

    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
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
    use super::{is_power_of_two, sort, sort_by};
    use crate::SortOrder::*;

    // 構造体 Student を定義する
    // 構造体は関連する値を一つにまとめたデータ構造。複数のデータフィールドを持つ。
    // 構造体の定義ではフィールドの型を省略できない
    struct Student {
        first_name: String,  // first_name(名前) フィールド。
        second_name: String, // second_name(苗字) フィールド。
        age: u8,             // 年齢
    }

    // impl ブロックを使うと、対象の型に関連関数やメソッドを実装できる
    impl Strudent {
        // new 関数は関連関数にあたり、Student::new(...)の形式で呼び出します
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体 Student を初期化して返す。Selfは impl 対象の型（Student）の別名
            Self {
                // to_string メソッドで &str 型の引数から String 型の値を作る。
                first_name: first_name.to_string(), // first_name フィールドに値を設定
                last_name: last_name.to_string(), // last_name フィールドに値を設定
                age, // ageフィールドにage変数の値を設定
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
            sort_by(&mut x, &|a, b| a.age.cmp(b.age)),
            Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }


    fn sort_students_by_name_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(sort_by(&mut x,
                           &|a, b| a.last_name.cmp(&b.last_name)
                           // もし last_name が等しくない（LessまはたGreater）ならをれを返す
                           // last_name が等しい（Equal）ならfirst_nameを比較する
                           .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())

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
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        // let mut x = vec!["Rust", "is", "a", "system", "programming", "language", "that", "runs"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
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
}
