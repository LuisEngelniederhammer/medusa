use std::{collections::HashMap, time::Duration};

use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self, WebsocketContext};

enum Function {
    Authentication,
}
struct Package {
    function: Function,
}

struct PeerMap<'a> {
    pub peers: HashMap<String, &'a mut WebsocketContext<HermesBackendService>>,
}

struct HermesBackendService;

impl HermesBackendService {}

impl Actor for HermesBackendService {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for HermesBackendService {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(_text)) => {
                ctx.run_interval(Duration::from_secs(5), |_act, ctx| {
                    //Do this in started within the Actor implementation for HermesBackendService
                    &self.peers.insert("some-transaction-id".to_string(), ctx); //fixme shade
                });
            }
            _ => (),
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<PeerMap<'static>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(HermesBackendService {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(PeerMap {
                peers: HashMap::new(),
            }))
            .route("/hermes/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
