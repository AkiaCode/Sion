#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{Request, request::{self, FromRequest, Outcome}, response::Redirect};
use serde_yaml::Value;

#[macro_use] extern crate rocket;
#[derive(Debug)]
struct Req {
    uri: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Req {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        Outcome::Success(Req { uri: req.headers().get_one("Host").unwrap().to_string() })
    }
}

#[get("/")]
fn redirect(request: Req) -> Redirect {
   let uri=  link(&request.uri).unwrap();
   let port: String = match  uri.clone().parse::<usize>() {
       Ok(_) => "http://localhost:".to_owned() + &uri,
       Err(_) => {
           if uri.contains("http") {
               uri
           } else {
               "https://".to_owned() + &uri
           }
       }
   };
   //println!("{}", port);
   Redirect::to(port)
}

fn link(req: &str) -> Result<String, ()> {
    let file = std::fs::read_to_string("./Sion.yaml").unwrap_or_default();
    let config: Value = serde_yaml::from_str(&file).unwrap();
    let a = config["bounce"].as_mapping().unwrap();
     for(i, n) in a {
         if i.as_str().unwrap() == req {
            //println!("{:?}, {:?}", i, n);
            let uri = serde_yaml::to_string(&n).unwrap().trim_matches(|x: char| x == '\n' || x == '-').to_owned();
            return Ok(uri);
        }
     }
    return Err(());
}

fn main() {
    rocket::ignite().mount("/", routes![redirect]).launch();
}
