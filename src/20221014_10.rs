use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        x: [[u32; 2]; n],
    }

    let mut v = vec![0; d + 1];
    for i in 0..n {
        v[(x[i as usize][0] - 1) as usize] += 1;
        v[x[i as usize][1] as usize] -= 1;
    }

    let mut sum: i32 = 0;
    for i in 0..d {
        sum += v[i as usize];
        println!("{}", sum);
    }
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

// let m = s.chars().nth(i).unwrap() as i32 - 48;
