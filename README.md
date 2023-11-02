# Image Resizer

Image Rezier Cloudflare Worker that returns base64 encoded image data.

Feel free to use this API in your projects; please include pertinent information in the `User-Agent` header, such as project name or a URL.

## Supported Image Dimensions

- This worker also accepts dimensions for `width` and `height` within the formdata. 
- These dimensions should be powers of 2, ranging from 16px to 2048px. If no dimensions are specified, it will default to `128`, invalid dimensions will throw `400`.

## Usage

`/resize` - POST

- `image` - The image to be resized (required)
- `width` - The width of the image (optional)
- `height` - The height of the image (optional)

## Setup

- Clone repo `git clone https://github.com/wanderer-moe/img-resizer`
- Modify `wrangler.toml` to your needs
- Deploy w/ `wrangler deploy`

## Run locally

- `wrangler dev`

## Deploy

- `wrangler deploy`

## License

This repository is licensed under the [GNU Affero General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/) license — **You must state all significant changes made to the original software, make the source code available to the public with credit to the original author, original source, and use the same license.**
