use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut even: Vec<u32> = Vec::new();
    let mut odd: Vec<u32> = Vec::new();

    for i in 0..n {
        if a[i] % 2 == 0 {
            even.push(a[i]);
        } else {
            odd.push(a[i]);
        }
    }

    let max1: i32;
    let max2: i32;

    if even.len() == 0 {
        max1 = -1;
    } else if even.len() == 1 {
        max1 = even[0] as i32;
    } else {
        max1 = (even[0] + even[1]) as i32;
    }

    if odd.len() == 0 {
        max2 = -1;
    } else if odd.len() == 1 {
        max2 = -1;
    } else {
        max2 = (odd[0] + odd[1]) as i32;
    }

    if ((max1 == -1) && (max2 == -1)) || ((max1 == 0) && (max2 == -1)) {
        println!("-1");
    } else {
        let max = {
            if max1 > max2 {
                max1
            } else {
                max2
            }
        };
        println!("{}", max);
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
