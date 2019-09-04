use rocket::response::Responder;
use rocket::{Response, Request, Data, Outcome};
use rocket::http::{Status, ContentType};
use rocket::data::FromDataSimple;
use serde::{Serialize, Deserialize};

//#[derive(Responder)]
//#[response(status = 200, content_type = "json")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Resp{
    pub url: String

}

impl FromDataSimple for Resp{
    type Error = ();

    fn from_data(request: &Request, data: Data) -> Outcome<Self, (Status, Self::Error), Data> {
//       println!("{:?}", data);
        Outcome::Success(Resp{url: String::from("")})
    }
}
