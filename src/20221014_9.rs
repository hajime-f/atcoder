use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
        q: u32,
        x: [[u32; 2]; q],
    }

    let mut atari: Vec<u32> = Vec::new();
    let mut hazure: Vec<u32> = Vec::new();

    atari.push(0);
    hazure.push(0);
    for i in 0..n {
        if a[i as usize] == 1 {
            atari.push(atari[i as usize] + 1);
            hazure.push(hazure[i as usize]);
        } else {
            atari.push(atari[i as usize]);
            hazure.push(hazure[i as usize] + 1);
        }
    }

    for i in 0..q {
        let m1 = atari[x[i as usize][1] as usize] - atari[(x[i as usize][0] - 1) as usize];
        let m2 = hazure[x[i as usize][1] as usize] - hazure[(x[i as usize][0] - 1) as usize];

        if m1 > m2 {
            println!("win");
        } else if m1 == m2 {
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

// let m = s.chars().nth(i).unwrap() as i32 - 48;
