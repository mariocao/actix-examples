use actix::dev::{MessageResponse, ResponseChannel};
use actix::*;
use futures::Future;

enum Requests {
    Ping,
    Pong,
    Die,
}

enum Responses {
    GotPing,
    GotPong,
    GotDie,
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
struct Game {
    counter: usize,
}

// Provide Actor implementation for our actor
impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Messages` enum
impl Handler<Requests> for Game {
    type Result = Responses;

    fn handle(&mut self, msg: Requests, _ctx: &mut Context<Self>) -> Self::Result {
        self.counter += 1;
        match msg {
            Requests::Ping => {
                println!("Ping received. Counter: {:?}", self.counter);

                Responses::GotPing
            }
            Requests::Pong => {
                println!("Pong received. Counter: {:?}", self.counter);

                Responses::GotPong
            }
            Requests::Die => {
                println!("Die received. Counter: {:?}", self.counter);
                _ctx.stop();

                Responses::GotDie
            }
        }
    }
}

fn main() {
    let sys = System::new("test");
    let addr = Game { counter: 0 }.start();

    println!("--Do send--");
    addr.do_send(Requests::Ping);
    addr.do_send(Requests::Pong);

    // // Send Ping message.
    // // send() message returns Future object, that resolves to message result
    println!("--Send with futures--");
    let ping_future = addr.send(Requests::Ping);
    let pong_future = addr.send(Requests::Pong);

    // Stop
    let die_future = addr.send(Requests::Die);

    // Spawn ping_future onto event loop
    Arbiter::spawn(
        ping_future
            .map(|res| match res {
                Responses::GotPing => println!("1: Ping received"),
                Responses::GotPong => println!("1: Pong received"),
                Responses::GotDie => println!("1: Die received"),
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
                Responses::GotDie => println!("2: Die received"),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    //Spawn die_future onto event loop
    Arbiter::spawn(
        die_future
            .map(|res| match res {
                Responses::GotPing => println!("3: Ping received"),
                Responses::GotPong => println!("3: Pong received"),
                Responses::GotDie => {
                    System::current().stop();
                    println!("3: Die received")
                }
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    sys.run();
}
