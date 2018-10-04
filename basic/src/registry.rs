use actix::*;

#[derive(Message)]
struct Ping;

#[derive(Default)]
struct SystemActor;

impl Actor for SystemActor {
    type Context = Context<Self>;
}
impl actix::Supervised for SystemActor {}

impl SystemService for SystemActor {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("System Service started");
    }
}

impl Handler<Ping> for SystemActor {
    type Result = ();

    fn handle(&mut self, _: Ping, _ctx: &mut Context<Self>) {
        println!("ping received by system actor");
    }
}

struct MyActor1;

impl Actor for MyActor1 {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        let act = System::current().registry().get::<SystemActor>();
        act.do_send(Ping);
    }
}

#[derive(Default)]
struct ArbiterActor;

impl Actor for ArbiterActor {
    type Context = Context<Self>;
}
impl actix::Supervised for ArbiterActor {}

impl ArbiterService for ArbiterActor {
   fn service_started(&mut self, _ctx: &mut Context<Self>) {
      println!("Arbiter service started");
   }
}

impl Handler<Ping> for ArbiterActor {
   type Result = ();

   fn handle(&mut self, _: Ping, _ctx: &mut Context<Self>) {
      println!("ping received by arbiter actor");
   }
}

struct MyActor2;

impl Actor for MyActor2 {
   type Context = Context<Self>;

   fn started(&mut self, _: &mut Context<Self>) {
      // get MyActor1 addres from the registry
      let act = Arbiter::registry().get::<ArbiterActor>();
      act.do_send(Ping);
   }
}

fn main() {
    // initialize system
    System::run(|| {
        // Start MyActor1
        SystemActor.start();

        // Start MyActor2
        MyActor1.start();

        // Start MyActor3 in new Arbiter
        Arbiter::start(|_| {
            MyActor2
        });
    });
}