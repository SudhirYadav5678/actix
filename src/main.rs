use actix_web::{get, HttpServer, App, web::Path, Responder, HttpResponse};
use rhai::Engine;

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();
    
    // Registering dynamic variables
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);
    
    match engine.eval_file::<i64>("src/multiply.rhai".into()) {
        Ok(result) => HttpResponse::Ok().body(format!("{result}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();

    // Registering dynamic variables
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);
    
    match engine.eval_file::<i64>("src/add.rhai".into()) {
        Ok(result) => HttpResponse::Ok().body(format!("{result}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
    })
    .bind("127.0.0.1:8000")?  // Correctly format bind address
    .run()
    .await
}
