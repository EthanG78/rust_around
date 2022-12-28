use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdData {
    m: u64,
    n: u64,
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(gcd_index))
            .route("/gcd", web::post().to(gcd_calculate))
    })
    .bind(("127.0.0.1", 8080))
    .expect("error binding server to addr")
    .run()
    .expect("error running server")
}

fn gcd_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <h1>GCD calculator</h1>
                <form action="/gcd" method="post">
                <input type="text" name="m"/>
                <input type="text" name="n"/>
                <button type="submit">Compute</button>
                </form>
            "#,
        )
}

fn gcd_calculate(form: web::Form<GcdData>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Please supply values greater than zero...");
    }

    let response = format!("The gcd of {} and {} is <b>{}</b>",
        form.m, form.n, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while n > 0 {
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n = n % m;
    }
    m
}

