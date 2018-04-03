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

static SERVER_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {

    let rocket = rocket::ignite().mount("/", routes![index, resources]);

    let mut url = String::from("http://");
    {
        let config  = &rocket.config();
        url.push_str(&config.address);
        url.push_str(":");
        url.push_str(&String::from(config.port.to_string()));
    }

    let client_child = std::thread::spawn(move ||
                                          {
                                              std::thread::park();
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

    let server_child = std::thread::spawn(||
                                          {
                                              rocket.attach(rocket::fairing::AdHoc::on_launch(|_| {
                                                  SERVER_RUNNING.store(true, std::sync::atomic::Ordering::Relaxed);
                                              })).launch();
                                          });
    while !SERVER_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {};
    client_child.thread().unpark();

    client_child.join().unwrap();
    server_child.join().unwrap();
}
