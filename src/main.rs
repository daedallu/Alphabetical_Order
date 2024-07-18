use std::io::prelude::*;
use std::io::Write;
use std::path::PathBuf;
fn main(){
    use std::cmp::Ordering;
    let mut vec = Vec::new();
    let pattern = std::env::args().nth(1).expect("Sorry. This app requires arg\n ex.: 'cargo run -- option'.\nFor more info, tip cargo run -- miniman.");
    let pattwo = std::env::args().nth(2).expect("Sorry. This app requires arg\n ex.: 'cargo run -- option'.\nFor more info, tip cargo run -- miniman.");
    let path = std::env::args().nth(3).expect("Requuires a path for file.");
    struct Cli {pattern: String, path: std::path::PathBuf,}
    let args = Cli{ pattern: pattern.clone(), path: std::path::PathBuf::from(path), };
    if pattern == "miniman"{
        println!("WELCOME TO STRING ORDERER\nFor Ascending order, tip 'cargo run -- asc';\nFor descending order, tip 'cargo run -- desc';\nAnd tip 'exit' for out.");
        }
    else if pattern == "read" && pattwo == "r" {
        readit(args.path.clone())
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
    if pattwo == "make"{
    let mut kick = String::new();
    std::io::stdin().read_line(&mut kick).expect("Sorry.NOT read the line.");
    let trimd_kick = kick.trim();
    let mkfile = createfile(args.path.clone(), trimd_kick.clone().to_string());
    for i in vec{
        let writes = writefile(i.clone(), args.path.clone());
        println!("{}", i.trim());
    }
}else if pattwo == "a"{
    for i in vec{
        let writes = writefile(i.clone(), args.path.clone());
        println!("{}", i.trim());
        
    }}
}
}
fn writefile(item: String, path: PathBuf) -> std::io::Result<()>{
    //let file = std::fs::File::create("/home/bettoshell/foolist.txt").append(true);
    //let pathstr = path.display().to_string();
    let mut file = std::fs::File::options().append(true).open(path)?;
    //file?.write_all(item.as_bytes())?; Ok(())  
    writeln!(&mut file, "{}", item.trim())?; Ok(())
    
    }
fn createfile(path: PathBuf, item: String) -> std::io::Result<()>{
    let file = std::fs::File::create(path);
    file?.write_all(item.as_bytes())?; Ok(()) 
}

fn readit(file_path: PathBuf){
    let fmtd = file_path.display();
    println! ("In the file {fmtd}");
    let f_contents = std::fs::read_to_string(file_path).expect("Sorry. An error occurred.");
    println!("With text:\n{f_contents}");
    }
