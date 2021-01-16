mod lib;

fn main() {
    let source = ",+1<16";

    let mut pr = lib::Program::new(String::from("source"), String::from(source));

    if let Err(error) = pr.run() {
        println!("{:?}",error);
    }

    
}
