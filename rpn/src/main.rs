fn main() {
    // exp 変数をRPN形式の字列に束縛する
    // この RPNは数式 6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6 と等しい
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);
    debug_assert_eq!("26.2840", format!("{:.4}", ans));
    println!("{} = {:.4}", exp, ans);
}

// RPN 形式の文字列 exp を受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64 {
    // 変数 stack を空のスタックに束縛する
    // stack はミュータブル (mutable, 可変)な変数で、値の変更を許す
    let mut stack = Vec::new();

    // exp のAsMutをスペースで分割し、tokenをそれら順に束縛する
    // 要素がなくなるまで繰り返す
    // exp.split_whitespace() は、メソッド呼び出し構文と呼ばれ、コンパイル時に split_whitespace(&exp) という関数呼び出しとして評価されます
    for token in exp.split_whitespace() {
        // token が f64型の数値ならスタックに積む
        // たとえば "6.1" なら成功しますので Ok(6.1)が返されます
        // それを if let 式で受けることで、数値の場合は条件が成立し、ベクタの push メソッドでその値をスタックの最後に追加します

        match token.parse::<f64>() {
            Ok(num) => {
                stack.push(num);
            }
            _ => {
                // token が数値でないなら、演算子なのか調べる
                match token {
                    // token が演算子なら apply2 関数で計算する
                    // |x, y| x + y はクロージャ
                    // 引数x, yを取り, x + y を計算して答えを返す
                    "+" => apply2(&mut stack, |x, y| x + y),
                    "-" => apply2(&mut stack, |x, y| x - y),
                    "*" => apply2(&mut stack, |x, y| x * y),
                    "/" => apply2(&mut stack, |x, y| x / y),
                    _ => panic!("Unknown operator: {}", token),
                }
            }
        }
    }

    // スタックから数値を一つ取り出す。失敗したらエラーを起こして終了する
    // この関数の最後の式の右側にセミコロンがついていないことに注目してください。
    // こうするとその式が返した値を、関数の戻り値として呼び出し元へ返せます
    stack.pop().expect("Stack underflow")
}

// スタックから数値を2つ取り出し、Ｆ型のクロージャfunで計算し、結果をスタックに積む
// <F> はこの関数がジェネリクスで、型パラメータとしてFを取ることを意味する
// F は where節で指定したトレイと境界を満たす型なら、どれにでもなれます
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    // 変数yとxをスタックの最後2要素に束縛する
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャfunで計算し、その結果に変数ｚを束縛する。
        let z = fun(x, y);
        // 変数zの値をスタックに積む
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかった時はエラーを起こして終了する
        panic!("Stack underflow");
    }
}
