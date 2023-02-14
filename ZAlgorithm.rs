// https://judge.yosupo.jp/problem/zalgorithm

fn main(){
    let mut word = String::new();
    let _getsridofwarning = std::io::stdin().read_line(&mut word);
    word = word.trim().to_string();

    let answers = "";
    for i in 0..word.len() {
        let mut r = 0;
        while (r+i)<word.len() && word.chars().nth(r+i).unwrap()==word.chars().nth(r).unwrap(){
            r = r+1;
        }
        let answers = &format!("{}{} ", answers ,r);
        print!("{}", answers)
    }
    println!("")
}