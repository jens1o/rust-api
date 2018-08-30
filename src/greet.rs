use model::user::User;
use rocket::http::Cookie;
use rocket::http::Cookies;
use rocket_contrib::Json;

#[get("/?<user>")]
fn route(user: User, mut cookies: Cookies) -> Json {
    let cookie = cookies.get_private("test");

    if let Some(ref cookie) = cookie {
        println!("Cookie exists! {}", cookie);
    } else {
        println!("Cookie does not exist!");
        cookies.add_private(Cookie::new("test", "123456"));
        println!("Cookie has been set.");
    }

    Json(json!({
        "message": user.greet()
    }))
}
