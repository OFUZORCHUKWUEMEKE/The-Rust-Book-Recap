use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main(){
    let greeting_file_result = File::open("hello.txt");


    let greeting_file = match greeting_file_result{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file: {e:?}")
            },
            _=>{
                panic!("Problem opening the file: {error:?}");
            }
        }
    }

    let greetinfg_file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == Error::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {error:?}");
            })
        }else{
            panic!("Problem opening the file: {error:?}");
        }
    })

    let greeting = File::open("hello.txt").expect("hello.txt should be included in this project");
}

