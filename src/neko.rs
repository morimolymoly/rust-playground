
pub struct Neko {
    pub name: String,
    pub age: i64,
}

pub trait Animal {
    fn Die(&self) {
        println!("die");
    }
}

impl Neko {
    pub fn Naku(&self) {
        println!("my name is {} {} yo", self.name, self.age);
    }

    pub fn GetOld(&mut self) {
        println!("GET OLD");
        self.age+=1;
    }
}

impl Animal for Neko {
    fn Die(&self) {
        println!("{} < ぐえー死んだンゴ", self.name);
    }
}
