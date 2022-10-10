fn main() {
    let (m1, m2) = input_pattern01();

    println!("{:?}", m1);
    println!("{:?}", m2);
}

fn input_pattern01() -> (Vec<i32>, Vec<i32>) {
    //
    // 最初に要素数を入力
    // 次に複数の値を入力
    // 3
    // 1 2
    // 3 4
    // 5 6
    //
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let n = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        n
    };

    let mut m1: Vec<i32> = Vec::new();
    let mut m2: Vec<i32> = Vec::new();

    for _ in 0..n {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };

        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();

        m1.push(n);
        m2.push(m);
    }

    return (m1, m2);
}
