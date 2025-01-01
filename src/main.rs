
use actix_web::{get, Responder,HttpServer,App,web::Path};
use rhai::Engine;
#[get("/multiply/{num1}/{num2}")]
async fn multiply(path:Path<(i64,i64)>)-> impl Responder {
    let (num1,num2)=path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("num1",move || num1);
    engine.register_fn("num2",move || num2);
    let result = engine.eval_file::<i64>("src/multipply.rhai".into()).unwrap();
    format!("multiply result {:?}", result)
}
#[get("/add/{num1}/{num2}")]
async fn add(path:Path<(i64,i64)>)-> impl Responder {
   let (num1,num2)= path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);
    let result :i64 = engine.eval_file::<i64>("src/add/rhai.rhai".into()).unwrap();
    format!("add result {:?}", result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new().service(multiply).service(add)}).bind(("127.0.0.1"),8000).unwrap().run().await

}

