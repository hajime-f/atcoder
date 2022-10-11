use proconio::input;

fn main() {
    input! {
        n: u32,
        mut d: [i32; n],
    }

    d.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut ans = 0;
    for i in 0..n {
        if i == 0 {
            ans += 1;
        } else {
            if d[i as usize] < d[(i - 1) as usize] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

// let mut v = vec![0; 2];  // 要素数2のベクタを0で初期化

// a.sort_by(|a, b| b.partial_cmp(a).unwrap());  // 降順にソート

// input! {
//     n: u32,
//     a: [i32; n],
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

// fn input_pattern01() -> (Vec<i32>, Vec<i32>) {
//     //
//     // 最初に要素数を入力
//     // 次に複数の値を入力
//     // 3
//     // 1 2
//     // 3 4
//     // 5 6
//     //
//     let s = {
//         let mut s = String::new();
//         std::io::stdin().read_line(&mut s).unwrap();
//         s.trim_end().to_owned()
//     };

//     let n = {
//         let mut ws = s.split_whitespace();
//         let n: i32 = ws.next().unwrap().parse().unwrap();
//         n
//     };

//     let mut m1: Vec<i32> = Vec::new();
//     let mut m2: Vec<i32> = Vec::new();

//     for _ in 0..n {
//         let s = {
//             let mut s = String::new();
//             std::io::stdin().read_line(&mut s).unwrap();
//             s.trim_end().to_owned()
//         };

//         let mut ws = s.split_whitespace();
//         let n: i32 = ws.next().unwrap().parse().unwrap();
//         let m: i32 = ws.next().unwrap().parse().unwrap();

//         m1.push(n);
//         m2.push(m);
//     }

//     return (m1, m2);
// }
