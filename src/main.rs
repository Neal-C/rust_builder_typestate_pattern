use prelude::*;

mod errors;
mod prelude;
mod typestate;
use typestate::{Request, RequestBuilder};
fn main() -> Result<()> {
    let request_builder: RequestBuilder<typestate::Url, typestate::Method, typestate::NotSealed> =
        RequestBuilder::new()
            .url("https://interview-me.com")
            .method("GET");

    let request: Request = request_builder
        .header("Token", "uuid.expiration.signature")
        .body("Just interview me ")
        .header("Hire", "yes")
        .seal()
        .build()?;

    println!("{request:#?}");

    Ok(())
}
