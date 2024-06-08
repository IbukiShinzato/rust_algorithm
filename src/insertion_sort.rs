use proconio::input;

//挿入ソート
pub fn run() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 1..n {
        //未ソート部分の先頭
        let v = a[i];

        //ソート済みの部分の末尾
        let mut j = i as isize - 1;
        while j >= 0 && a[j as usize] > v {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
        }

        a[(j + 1) as usize] = v
    }

    trace(&a, n);
}

fn trace(a: &[usize], n: usize) {
    let mut res = vec![];
    for i in 0..n {
        res.push(format!("{}", a[i]));
    }
    println!("{}", res.join(" "));
}
