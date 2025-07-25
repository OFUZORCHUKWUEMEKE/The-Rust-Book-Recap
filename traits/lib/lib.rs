struct Sheep{naked:bool ,name:&'static str}

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep{
    fn is_naked(&self)->bool{
        self.naked
    }

    fn shear(&mut self){
        if self.is_naked(){
            println!("{} is already naked....",self.name());   
        }else{
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep{
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name:false }
    }

    fn name(&self)->&'static str{
        self.name
    }

    fn noise(&self)->&'static str{
        if self.is_naked(){
            "baaaaaah?"
        }else{
            "baaaaah!"
        }
    }

    fn talk(&self){
        println!("{} pauses briefly... {}",self.name,self.noise())
    }

    fn commit(&self){
        println!("commited code to codebase to repository successfully")
    }
}