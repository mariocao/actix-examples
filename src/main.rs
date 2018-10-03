use actix::{Actor, Arbiter, Context, Handler, Message, System};
use futures::{future, Future};

// this is our Message
struct SumRequest(usize, usize);

// we have to define the response type for `Sum` message
impl Message for SumRequest {
    type Result = usize;
}

// Actor definition
struct Summator;

impl Actor for Summator {
    type Context = Context<Self>;
}

// now we need to define `MessageHandler` for the `Sum` message.
impl Handler<SumRequest> for Summator {
    type Result = usize;   // <- Message response type

    fn handle(&mut self, msg: SumRequest, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

fn main() {
    let sys = System::new("test");

    let addr = Summator.start();
    let res = addr.send(SumRequest(10, 5));  // <- send message and get future for result
    
    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("SUM: {}", result),
            _ => println!("Something wrong"),
        }
        
        System::current().stop();
        future::result(Ok(()))
    }));

    sys.run();
}