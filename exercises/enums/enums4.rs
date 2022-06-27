#![allow(proc_macro_derive_resolution_fallback)]
// enums4.rs
// Address all the TODOs to make the tests pass!

enum State {
    Index(String),
    SetDog(Option<Dog>),
    SetCat(Option<Cat>),
    Active,
    SettleAt(Point),
    Remove,
}

#[derive(PartialEq, Debug)]
struct Point {
    x: u8,
    y: u8,
}

struct AnimalVillage {
    id: String,
    cat: Option<Cat>,
    dog: Option<Dog>,
    status: bool,
    location: Point,
}

#[derive(PartialEq, Debug, Clone)]
struct Cat {
    name: String,
    color: String,
    voice: String,
}

#[derive(PartialEq, Debug, Clone)]
struct Dog {
    name: String,
    color: String,
    voice: String,
}

impl Cat {
    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_voice(&mut self) {
        self.voice = String::from("Meow");
    }

    fn get_name(&mut self) -> String {
        self.name.to_string()
    }

    fn speak(&mut self) {
        println!("{} is speaking {}",self.get_name(), self.voice);
    }
}


impl Dog {
    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_voice(&mut self) {
        self.voice = String::from("Woff");
    }

    fn get_name(&mut self) -> String {
        self.name.to_string()
    }

    fn speak(&mut self) {
        println!("{} is speaking {}", self.get_name(), self.voice);
    }
}

impl AnimalVillage {
    fn set_id(&mut self, id: String){
        self.id = id;
    }

    fn add_cat(&mut self, cat: Option<Cat>) {
        self.cat = cat;
    }

    fn add_dog(&mut self, dog: Option<Dog>) {
        self.dog = dog;
    }

    fn set_remove(&mut self) {
        self.status = false;
    }

    fn set_location(&mut self, point: Point) {
        self.location = point;
    }

    fn set_active(&mut self) {
        self.status = true;
    }

    fn process(&mut self, state: State) -> Option<AnimalVillage> {
        match state {
            State::SetDog(dog) => {
                self.add_dog(dog);
                None
            },
            State::SetCat(cat) => {
                self.add_cat(cat);
                None
            },
            State::Index(index) => {
                self.set_id(index);
                None
                
            },
            State::Remove => {
                self.set_remove();
                None
            },
            State::SettleAt(p) => {
                self.set_location(p);
                None
            },
            State::Active => {
                self.set_active();
                None
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        // define Dog 
        let mut dog = Dog {name: String::from("Luc"), color: String::from("Null"), voice: String::from("Null")};
        dog.set_color(String::from("White"));
        dog.set_voice();
        // assert Dog
        assert_eq!(dog.color, "White");
        assert_eq!(dog.voice, "Woff");
        // define Cat 
        let mut cat = Cat {name: String::from("Dassy"), color: String::from("Null"), voice: String::from("Null")};
        cat.set_color(String::from("Black"));
        cat.set_voice();
        // assert Cat
        assert_eq!(cat.color, "Black");
        assert_eq!(cat.voice, "Meow");

        // settle new Village of dog
        let mut vildog = AnimalVillage { id: String::from("Dog Village"), cat: None, dog: None, status: false , location: Point { x: 0, y: 0}};
        vildog.process(State::SetDog(Some(dog.clone())));
        vildog.process(State::Index(String::from("Dog Village settled")));
        vildog.process(State::SettleAt(Point { x: 15, y: 25}));
        vildog.process(State::Active);

        assert_eq!(vildog.cat, None);
        assert_eq!(vildog.dog, Some(dog.clone()));
        assert_eq!(vildog.status, true);
        assert_eq!(vildog.location, Point {x: 15, y: 25});
        assert_eq!(vildog.id, String::from("Dog Village settled"));
        // settle new Village of cat
        let mut vilcat = AnimalVillage { id :String::from("Cat Village"), cat: None, dog: None, status: false, location: Point { x: 0, y: 0}};
        vilcat.process(State::SetCat(Some(cat.clone())));
        vilcat.process(State::Index(String::from("Cat Village settled")));
        vilcat.process(State::SettleAt(Point { x: 25, y: 15 }));
        vilcat.process(State::Active);

        assert_eq!(vilcat.cat, Some(cat.clone()));
        assert_eq!(vilcat.dog, None);
        assert_eq!(vilcat.status, true);
        assert_eq!(vilcat.location, Point { x: 25, y: 15 });
        assert_eq!(vilcat.id, String::from("Cat Village settled"));
        // settle new Village of combination all dog and cat
        let mut vilall = AnimalVillage { id :String::from("Combine Village"), cat: None, dog: None, status: false, location: Point { x: 0, y: 0}};
        vilall.process(State::SetDog(Some(dog.clone())));
        vilall.process(State::SetCat(Some(cat.clone())));
        vilall.process(State::Index(String::from("Combine Village settled")));
        vilall.process(State::SettleAt(Point { x: 10, y: 10}));
        vilall.process(State::Active);
        // assertment settlement Village
        assert_eq!(vilall.cat, Some(cat.clone()));
        assert_eq!(vilall.dog, Some(dog.clone()));
        assert_eq!(vilall.status, true);
        assert_eq!(vilall.location, Point { x:10, y: 10});
        assert_eq!(vilall.id, String::from("Combine Village settled"));
    }
}