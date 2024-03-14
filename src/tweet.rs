use actix_web::{
    get , HttpResponse,web
};

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/")]
pub async fn get() -> HttpResponse {
    let my_variable: &str = "Hello, world!";
    HttpResponse::Ok().body(my_variable)

}

#[get("/tweets/{id}")]
pub async fn get_by_id(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(id.to_string())
}