use actix::{Actor, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
       println!("I am alive!");
       System::current().stop(); // <- stop system
    }
}

fn main() {
    let system = System::new("test");

    let _ = MyActor.start();

    system.run();
}