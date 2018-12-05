extern crate cdrs;
extern crate json;
extern crate uuid;



struct User {
    id: i32,
    name: String,
}


enum UserEvent {
    CreateUser(User),
    UpdateUser(User),
    DeleteUser(User),
}

impl UserEvent {
    fn create_user(name: &str) -> UserEvent {
        UserEvent::CreateUser(User {
            id: 534,
            name: name.to_owned(),
        })
    }

    fn update_user(name: &str) -> UserEvent {
       // println!("{}",name);
        UserEvent::UpdateUser(User {
            id: 534,
            name: name.to_owned(),
        })
    }

    fn delete_user(name: &str) -> UserEvent {
        UserEvent::DeleteUser(User {
            id: 534,
            name: name.to_owned(),
        })
    }
}

#[derive()]
struct UserAggregate {
    id: i32,
    name: String,
    version: i32,
}

impl UserAggregate {

    fn new(name: &str) -> UserAggregate {
        UserAggregate {
            id: 534,
            name: name.to_owned(),
            version: 1,
        }
    }

    fn create_user(&self,name: &str) -> Result<Vec<UserEvent>, String> {
        Ok(vec![UserEvent::create_user(&name)])
    }

    fn update_user(&self,name: &str) -> Result<Vec<UserEvent>, String> {
        Ok(vec![UserEvent::update_user(&name)])
    }

    fn delete_user(&self,name: &str) -> Result<Vec<UserEvent>, String> {
        Ok(vec![UserEvent::delete_user(&name)])
    }
}


trait Aggregate {
    type Person;
    fn version(&self) -> i32;
    fn apply(self, evt: &Self::Person) -> Self where Self: Sized;
}


impl Aggregate for UserAggregate {
    type Person = UserEvent;

    fn version(&self) -> i32 {
        self.version
    }

    fn apply(mut self, evt: &UserEvent) -> UserAggregate {
     //   println!("{}",self.name);
       self.version +=1;
      //  println!("{}",self.version);
        self.name = match evt {
            &UserEvent::UpdateUser(User{ref name, ..}) => name,
            /* &UserEvent::CreateUser(User) => self.name,
             &UserEvent::DeleteUser(User) => self.name,*/
            _ => &self.name
        }.parse().unwrap();
        self
    }
}

fn main() {
    let user = UserAggregate::new("abhi");
    let u = user.apply(&UserEvent::update_user("sachin"));
    let a = u.apply(&UserEvent::update_user("amita"));
    println!("{}",a.name);

   /* println!("{}",u.name);
    println!("{}",u.version);*/
    println!("Hello, world! {}",a.create_user(&amita));
}
