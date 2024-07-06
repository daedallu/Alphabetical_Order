fn main(){
    use std::cmp::Ordering;
    let mut vec = Vec::new();
    let pattern = std::env::args().nth(1).expect("Sorry. This app requires arg\n ex.: 'cargo run -- option'.\nFor more info, tip cargo run -- miniman.");
    if pattern == "miniman"{
        println!("WELCOME TO STRING ORDERER\nFor Ascending order, tip 'cargo run -- asc';\nFor descending order, tip 'cargo run -- desc';\nAnd tip 'exit' for out.");
        }
        else{
    loop{
        println!("INSERT A STRING, PLEASE\n> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Sorry.NOT read the line.");
        let trimd = input.trim();
        if trimd == "exit"{
        break;
        }else{
        vec.push(input.clone().to_string())}
    }
    if pattern == "desc"{
        vec.sort_by(|a,b| if a>b{Ordering::Greater} else if a<b{Ordering::Less} else{Ordering::Equal});
        vec.sort_by(|a,b| b.cmp(a));
        println!("STRINGS IN DESCENDING ORDER:");}
        
    else if pattern == "asc"{
        vec.sort_by(|a,b| if a>b{Ordering::Greater} else if a<b{Ordering::Less} else{Ordering::Equal});
        vec.sort_by(|a,b| a.cmp(b));
        println!("STRINGS IN ASCENDING ORDER:");}
        

    //println!("{:?}", vec);
    for i in vec{
        println!("{}", i.trim());
    }
}
}
