use std::collections::HashMap;
use std::io::stdin;
use std::convert::TryInto;

fn main(){

    let mut snumcases = String::new();
    stdin().read_line(&mut snumcases);
    let numcases = snumcases.trim().parse::<i64>().unwrap();

    for nnn in 0..numcases{

        let mut snumpeople = String::new();
        stdin().read_line(&mut snumpeople);
        let numpeople = snumpeople.trim().parse::<i64>().unwrap();
        let mut people: HashMap<String, i64> = HashMap::new();

        // Upper = 1, Lower = 0, with farther down = higher place
        for i in 0..numpeople{
            let mut line = String::new();
            stdin().read_line(&mut line);
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let mut name = String::from(parts[0]);
            name.remove(name.len()-1);

            let mut cval = 0;
            let class = parts[1].split("-").collect::<Vec<_>>();
            for n in 0..class.len(){
                if(class[n]=="upper"){
                    cval = cval+2*10_usize.pow((n+1).try_into().unwrap())
                }
                else if (class[n]=="middle"){
                    cval = cval+10_usize.pow((n+1).try_into().unwrap())
                }
            }
            people.insert(name, cval.try_into().unwrap());
        }
        print_sorted(people);
    }
}

fn print_sorted(people: HashMap<String, i64>) -> () {
    let mut inordernames = Vec::new();
    
}