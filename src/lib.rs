use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    expression : String,
    file_name : String,
    env_var: u8,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not Enough Arguments. ");
        }
        args.next();
        let exp = match args.next() {
            Some(x) => x,
            None => return Err("Didn't Get a query Expression Parameter."),
        };
        let f_n = match args.next(){
            Some(f) => f,
            None => return Err("Didn't Get a File Name Parameter."),
        };
        let var:u8;
        let arg_3:&str = &(match args.next(){
            Some(v) => v,
            None => "0".to_string(),
        });
        match arg_3 {
            "-cs" => {
                env::set_var("CASE-SENSETIVE", "true");
                var = 1;
            },
            "-ci" =>{
                env::set_var("CASE-SENSETIVE","false");
                var = 2;
            } 
            "--help" =>{
                var = 3;
            },

            "0" => {
                var = 0
            },

            _ => {
                return Err("Incorrect Argument. ");
            }
        }
        return Ok(
            Config {
                expression : exp,
                file_name : f_n,
                env_var : var,
            }
        );
        
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        
        if self.env_var == 0 {
            self.run_without_env()
        }else{
            self.run_with_env(self.env_var)
        }
    }

    fn search<'a>(&self,contents:&'a str) -> Vec<&'a str>{
        contents
            .lines()
            .filter(|line| line.contains(&self.expression))
            .collect()
    }

    fn case_insensitive<'a>(& self,contents:&'a str) -> Vec<&'a str>{
        contents
            .lines()
            .filter(
                |line|
                    line
                        .to_lowercase()
                        .contains(&self.expression.to_lowercase())
                )
            .collect()
    }

    fn run_without_env(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_name)?;

        let case_sensitive:bool = env::var("CASE-SENSETIVE").unwrap_or_else(|_err|{
            "true".to_string()
        }) == "true";

        let res = {
            if case_sensitive {
                Config::search(self,&contents)
            }else{
                Config::case_insensitive(self,&contents)
            }
        };
        
        if res.len() != 0{
            println!("\nFound {} Results\n",res.len());
            for line in Config::search(&self,&contents){
                if !line.is_empty(){
                    println!("{}",line);
                }
            }
        }else{
            println!("Can't Find {} in {}",self.expression,self.file_name);
        }

        Ok(())
    }
    fn run_with_env(&self,env_var:u8) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_name)?;

        let res:Vec<&str>;
        match env_var{
            1 => {res = Config::search(self,&contents);},
            2 => {res = Config::case_insensitive(self,&contents);},
            3 => {
                Config::run_help();
                return Ok(());
            },
            _ => {
                return Ok(());
            }
        }
        
        if res.len() != 0{
            println!("\nFound {} Results\n",res.len());
            for line in res{
                if !line.is_empty(){
                    println!("{}",line);
                }
            }
        }else{
            println!("Can't Find {} in {}",self.expression,self.file_name);
        }

        Ok(())
    }

    fn run_help(){
        let help = "\
        \nMini-Grep is an easy to use expression finder in File
        
        To Use Mini-Grep you need atleast 2 Arguments <Expression_to_find> <File_name>
        
        Thrid Argument is used to Set Enivorment Variable Or for Help
        
        -cs => Searches File In Case-Sensitive Way. 
        -ci => Searches File In Case-Insensitive Way. 
        --help => Used To Print Help Text.\n\n";

        print!("{}",help);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let con = Config{
            expression : "duct".to_string(),
            file_name : "nun".to_string(),
            env_var : 0,
        };

        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."],con.search(contents));

    }

    #[test]
    fn case_sensitive(){
        let con = Config{
            expression : "rUsT".to_string(),
            file_name : "null".to_string(),
            env_var : 0,
        };

        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three
        Trust Me.";

        assert_eq!(vec!["Rust:","Trust Me."],con.case_insensitive(contents));
    }
}