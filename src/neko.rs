
pub struct Neko {
    pub name: String,
    pub age: i64,
}

pub struct Inu {
    pub name: String,
    pub age: i64,
}

pub fn returnNekoOrCat(neko: bool) -> Box<dyn Animal> {
    if neko {
        Box::new(Neko{name: String::from("neko"), age:100,})
    } else {
        Box::new(Inu{name: String::from("inu"), age:100,})
    }
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

impl Inu {
}

impl Animal for Inu {
    fn Die(&self) {
        println!("{} < ワン!(死)", self.name);
    }
}
