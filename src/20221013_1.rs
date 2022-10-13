use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32; n],
    }

    println!("{}", search(n, x, a) + 1);
}

fn search(n: i32, y: i32, a: Vec<i32>) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = n - 1;

    while l <= r {
        let m: i32 = (l + r) / 2;
        if y < a[m as usize] {
            r = m - 1;
        } else if y == a[m as usize] {
            return m;
        } else if y > a[m as usize] {
            l = m + 1;
        } else {
        }
    }

    return -1;
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
