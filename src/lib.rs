use serde_json::json;
use worker::*;

mod resize;
mod utils;

fn log_request(req: &Request) {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            let git_repo = ctx.var("WORKERS_RS_GIT_REPO")?.to_string();
            Response::from_json(
                &json!({ "version": version, "git_repo": git_repo, "endpoint": {
                    "resize": "/resize",
                    "method": "POST",
                    "accepts": "multipart/form-data",
                    "description": "Resize an image to a given width and height",
                    "params": {
                        "file": {
                            "type": "file",
                            "description": "The image file to resize",
                            "required": "true",
                        },
                        "width": {
                            "type": "text",
                            "description": "New image width, must be ^ 2 from 16 to 1024",
                            "required": "false",
                            "default": "128",
                        },
                        "height": {
                            "type": "text",
                            "description": "New image height, must be ^ 2 from 16 to 1024",
                            "required": "false",
                            "default": "128",
                        }
                    }
                }
                }),
            )
        })
        .post_async("/resize", |mut req, _ctx| async move {
            let form = req.form_data().await?;

            let width = match form.get("width") {
                Some(FormEntry::Field(field)) => field.parse::<u32>().ok(),
                _ => None,
            };

            let height = match form.get("height") {
                Some(FormEntry::Field(field)) => field.parse::<u32>().ok(),
                _ => None,
            };

            let file = match form.get("file") {
                Some(FormEntry::File(file)) => Some(file),
                _ => return Response::error("Bad Request", 400),
            };

            // if width or height is not provided, we default to 128 as stated in the readme
            let width = width.unwrap_or(128);
            let height = height.unwrap_or(128);

            if !(resize::validate_size(width).await) || !(resize::validate_size(height).await) {
                return Response::error(
                    "Invalid Width or Height Parameter: must be ^ 2 from 16 to 1024.",
                    400,
                );
            }

            if let Some(file) = file {
                let resized_image = resize::resize_image(file, width, height).await?;
                return Response::from_json(
                    &json!({ "image": resized_image, "width": width, "height": height }),
                );
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
