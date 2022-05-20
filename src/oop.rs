pub struct Sus {
    is_sus: bool
}
impl Sus {
    pub fn check_sus(&self) {
        match self.is_sus {
            true => println!("sus"),
            false => println!("not sus"),
        }
    }
}
fn main() {
    let mut crewmate = Sus {is_sus: true};
    crewmate.is_sus = false;
    crewmate.check_sus()
}

//returns with sus if is_sus is true and not sus if is_sus is false using a oop function 