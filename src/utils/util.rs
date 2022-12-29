use{
    actix_web::HttpResponse,
    serde::{Deserialize,Serialize},

};

#[derive(Debug, Deserialize, Serialize)]
pub struct NTF {
    message: String,
}


impl NTF {
    pub fn new(message: String) -> Self {
        Self{
            message
        }
    }
}

pub enum Resp<T>{
    Ok(T),
    Notfound(T),
    Created(T),

}

impl<T: Serialize> Resp<T> {
    pub fn get_response(&self) -> HttpResponse {
        match self {
            Resp::Ok(payload) => HttpResponse::Ok()
            .content_type("application/json")
            .json(payload),
            Resp::Notfound(message) => HttpResponse::BadRequest()
            .content_type("application/json")
            .json(message),
            Resp::Created(payload) => HttpResponse::Created()
            .content_type("application/json")
            .json(payload),
        }
    }
}