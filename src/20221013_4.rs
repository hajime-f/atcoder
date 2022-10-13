use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: u32,
        h: [i32; n],
    }

    let mut dp: Vec<i32> = Vec::new();

    dp.push(0);
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        dp.push(cmp::min(
            dp[(i - 1) as usize] + (h[i as usize] - h[(i - 1) as usize]).abs(),
            dp[(i - 2) as usize] + (h[i as usize] - h[(i - 2) as usize]).abs(),
        ));
    }

    println!("{}", dp[(n - 1) as usize]);
}

// let mut v = vec![0; 2];  // 要素数2のベクタを0で初期化
// let mut v: Vec<i32> = Vec::new();

// let a = String::new();  // 新しい空のStringを生成
// let a = String::from("This is a "); // 文字列を代入しながら初期化
// let a = "This is a ".to_string();

// a.sort_by(|a, b| b.partial_cmp(a).unwrap());  // 降順にソート

// let base = 2;
// n / base.pow(i)) % 2 // nを２進数に変換

// input! {
//     n: u32,
//     a: [i32; n],
// }

// input! {
//     n: usize,
//     m: usize,
//     a: [[i32; n]; m],
// }

// use proconio::input;
// use proconio::marker::Chars;

// fn main() {
//     input! {
//         c: Chars,
//     }
//     let num1 = c[0] as i32 - 48;
//     let num2 = c[1] as i32 - 48;
//     let num3 = c[2] as i32 - 48;

//     println!("{:?}", num1 + num2 + num3);
// }
