#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate web_view;

#[get("/")]
fn index() -> rocket::response::content::Html<String>
{
    rocket::response::content::Html(String::from(include_str!("../resources/html/index.html")))
}

#[get("/resources/<file..>")]
fn resources(file: std::path::PathBuf) -> Option<rocket::response::NamedFile>
{
    rocket::response::NamedFile::open(std::path::Path::new("resources/")
                                      .join(file))
    .ok()
}

fn main() {

    let rocket  = rocket::ignite().mount("/", routes![index, resources]);
    let mut url = String::from("http://");
    {
        let config  = &rocket.config();
        url.push_str(&config.address);
        url.push_str(":");
        url.push_str(&String::from(config.port.to_string()));
    }

    let server_child = std::thread::spawn(move ||
                                          {
                                              rocket.launch();
                                          });

    let client_child = std::thread::spawn(move ||
                                          {
                                              let title       = "self-hosting Web App example";
                                              let size        = (800, 600);
                                              let resizable   = true;
                                              let debug       = true;
                                              let init_cb     = move |_webview| {};
                                              let frontend_cb = move |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
                                              let userdata    = ();
                                              web_view::run(title,
                                                            &url,
                                                            Some(size),
                                                            resizable,
                                                            debug,
                                                            init_cb,
                                                            frontend_cb,
                                                            userdata);
                                          });

    client_child.join().unwrap();
    server_child.join().unwrap();
}
