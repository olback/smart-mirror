use rocket::fs::FileServer;

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", FileServer::from("static"))
        .launch()
        .await
    {
        eprintln!("{e}");
    }
}
