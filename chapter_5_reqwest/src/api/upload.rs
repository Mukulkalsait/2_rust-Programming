use reqwest::blocking::{
    Client,
    multipart::{self, Form},
};

use crate::{models::UploadResponse, utils::get_json};

pub fn upload_file(client: &Client) -> Result<UploadResponse, Box<dyn std::error::Error>> {
    let form: Form = multipart::Form::new().file("file", "cat.png")?;
    let res = client.post("http://localhost:3000/file").multipart(form).send()?;

    get_json(res)
}
