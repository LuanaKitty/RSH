use std::io::*;
use std::process::Command;
use std::path::Path;
use std::env;


fn main(){
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!(">>");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

	// everything after the first whitespace character 
     //     is interpreted as args to the command
	let mut parts = input.trim().split_whitespace();
     let command = parts.next().unwrap();
     let args = parts;

match command {
     
     "cd" =>{
	// take the home var of the current systen
     let home_dir = env::var("HOME").unwrap_or_else(|_|"/".to_string());
     //error handling if user dont have a HOME var
     let arg = args.peekable().peek().map(|x| x.to_string());
     
     let final_path = match arg{
          None => home_dir, // 'cd' goes to home
          Some(path) =>{
               if path == "~"{
                    home_dir
               }else if path.starts_with("~/"){
                    //substitues the ~ with the HOME dir
                    path.replacen("~", &home_dir, 1)
               }else{
                    path
               }
          }        
     };
     if let Err(e) = env::set_current_dir(Path::new(&final_path)) {
        eprintln!("Erro: {}", e);
    }
     
     },
     command =>{
          let mut child = Command::new(command).args(args).spawn().unwrap();
          child.wait();
     }
          
	 }
    }
}
