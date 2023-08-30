use base64::{engine::general_purpose, Engine as _};
use image::imageops::resize;
use image::io::Reader as ImageReader;
use serde_json::json;
use std::io::Cursor;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // panic hook which logs error messages into console
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            let git_repo = ctx.var("WORKERS_RS_GIT_REPO")?.to_string();
            Response::from_json(&json!({ "version": version, "git_repo": git_repo }))
        })
        // post /resize form, take in image and resize it
        .post_async("/resize", |mut req, _ctx| async move {
            let form = req.form_data().await?;
            if let Some(entry) = form.get("file") {
                return match entry {
                    FormEntry::File(file) => {
                        let bytes = file.bytes().await?;
                        let img = ImageReader::new(std::io::Cursor::new(bytes))
                            .with_guessed_format()
                            .unwrap()
                            .decode()
                            .unwrap();
                        let resized = resize(&img, 128, 128, image::imageops::FilterType::Nearest);
                        let mut buf = Cursor::new(Vec::new());
                        resized
                            .write_to(&mut buf, image::ImageOutputFormat::Png)
                            .unwrap();
                        let resized_image_data = buf.into_inner();

                        let resized_image_base64 =
                            general_purpose::STANDARD.encode(resized_image_data);

                        Response::from_json(&json!({ "image": resized_image_base64 }))
                    }
                    FormEntry::Field(_) => Response::error("Expected file", 400),
                };
            }
            Response::error("Bad Request", 400)
        })
        .get("/stats", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            let git_repo = ctx.var("WORKERS_RS_GIT_REPO")?.to_string();
            Response::from_json(&json!({ "version": version, "git_repo": git_repo }))
        })
        .run(req, env)
        .await
}
