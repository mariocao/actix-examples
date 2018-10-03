use actix::dev::{MessageResponse, ResponseChannel};
use actix::*;
use futures::Future;

enum Requests {
    Ping,
    Pong,
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Message for Requests {
    type Result = Responses;
}

// Define actor
struct MyActor;

// Provide Actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Messages` enum
impl Handler<Requests> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Requests, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Requests::Ping => Responses::GotPing,
            Requests::Pong => Responses::GotPong,
        }
    }
}

fn main() {
    let sys = System::new("test");

    // Start MyActor in current thread
    let addr = MyActor.start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let ping_future = addr.send(Requests::Ping);
    let pong_future = addr.send(Requests::Pong);

    // Spawn ping_future onto event loop
    Arbiter::spawn(
        ping_future
            .map(|res| match res {
                Responses::GotPing => println!("1: Ping received"),
                Responses::GotPong => println!("1: Pong received"),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    // Spawn pong_future onto event loop
    Arbiter::spawn(
        pong_future
            .map(|res| match res {
                Responses::GotPing => println!("2: Ping received"),
                Responses::GotPong => println!("2: Pong received"),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    sys.run();

}
