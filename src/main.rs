use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::{get, launch, routes};
use rocket::response::status::NotFound;


#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("site/static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path).await.map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, static_files])
}