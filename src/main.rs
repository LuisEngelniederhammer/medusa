#[macro_use] extern crate rocket;
use rocket::{Data, data::ToByteUnit};


#[post("/", format = "plain", data = "<data>")]
async fn upload(data: Data<'_>) -> String{
    let result_request_data = data
    .open(5.megabytes())
    .into_string().await;  

    match result_request_data{
        Ok(request_string) => {
            format!("You've sent {:?}", request_string)
        },
        Err(_) => String::from("error")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}