use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u32]; m],
    }

    let mut chk: Vec<Vec<bool>> = Vec::new();
    for _ in 0..n {
        chk.push(vec![false; n]);
    }

    for i in 0..m {
        for j in 0..a[i].len() {
            for k in (j + 1)..a[i].len() {
                chk[(a[i][j] - 1) as usize][(a[i][k] - 1) as usize] = true;
            }
        }
    }

    let mut ans = true;
    for i in 0..n {
        for j in (i + 1)..n {
            ans &= chk[i][j];
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

// let mut v = vec![0; 2];  // 要素数2のベクタを0で初期化
// let mut v: Vec<i32> = Vec::new();

// let a = String::new();  // 新しい空のStringを生成
// let a = String::from("This is a "); // 文字列を代入しながら初期化
// let a = "This is a ".to_string();

// a.sort_by(|a, b| b.partial_cmp(a).unwrap());  // 降順にソート

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
