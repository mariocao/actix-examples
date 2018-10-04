use actix;

use actix::actors::resolver::*;
use actix::prelude::*;

use tokio::io::*;
use tokio::codec::*;


struct TcpClientActor;

impl Actor for TcpClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("TcpClientActor started!");

        // The Connector actor can be used to perform DNS lookups, 
        // or connect to remote servers via TCP
        Resolver::from_registry() // return an Addr
            .send(Connect::host("towel.blinkenlights.nl:23")) // send Connect Message
            .into_actor(self) // convert to actor future
            .map(|res, _act, ctx| match res {
                Ok(stream) => {
                    println!("TcpClientActor connected!");

                    let (r, _w) = stream.split();
                    let line_reader = FramedRead::new(r, LinesCodec::new());
                    ctx.add_stream(line_reader);
                }
                Err(err) => {
                    println!("TcpClientActor failed to connected: {}", err);
                    ctx.stop();
                }
            })
            .map_err(|err, _act, ctx| {
                println!("TcpClientActor failed to connected: {}", err);
                ctx.stop();
            })
            .wait(ctx); //block until future is resolved!
    }
}

impl StreamHandler<String, std::io::Error> for TcpClientActor {
    fn handle(&mut self, line: String, _ctx: &mut Self::Context) {
        println!("{}", line);
    }
}

fn main() {
    let sys = actix::System::new("tcp-test");

    let _tcp_client = Arbiter::start(|_| TcpClientActor);

    sys.run();
}
