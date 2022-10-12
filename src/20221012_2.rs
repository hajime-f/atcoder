use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
        q: u32,
        x: [[u32; 2]; q],
    }

    let mut m1: Vec<u32> = Vec::new();
    let mut m2: Vec<u32> = Vec::new();

    m1.push(0);
    m2.push(0);
    for i in 1..=n {
        if a[(i - 1) as usize] == 1 {
            m1.push(m1[(i - 1) as usize] + 1);
            m2.push(m2[(i - 1) as usize]);
        } else {
            m1.push(m1[(i - 1) as usize]);
            m2.push(m2[(i - 1) as usize] + 1);
        }
    }

    for i in 0..q {
        let atari = m1[x[i as usize][1] as usize] - m1[(x[i as usize][0] - 1) as usize];
        let hazure = m2[x[i as usize][1] as usize] - m2[(x[i as usize][0] - 1) as usize];

        if atari > hazure {
            println!("win");
        } else if atari == hazure {
            println!("draw");
        } else {
            println!("lose");
        }
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
