#![allow(dead_code)]

// use eval::eval;
fn main()  {
    let rule = FalcexCore::new( "1 + 1".to_string() );

    println!( "{}", rule.syntax )


}


struct FalcexCore {
    syntax: String
}


impl FalcexCore {
    fn new(syntax : String )  -> FalcexCore {
        FalcexCore{ syntax }
       
    }
    
}