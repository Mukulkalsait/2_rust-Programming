use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

pub fn testing_rewest_func() {
    let http_request = Client::new();

    // Trying external websites on reqwest ---------------------------------------------------------------------------
    let vat = http_request.get("https://vihaanaitech.com").send();
    if vat.is_ok() {
        println!("{:#?}", vat.as_ref().ok().unwrap());
        println!("Body:{:#?}", &vat.ok().unwrap().text().unwrap());
    } else {
        println!("{:#?}", vat.err());
    }

    let body_in_res = r#"{"first_name":"Mukul"}"#;

    // sending data in request --------------------------------------------------------------------------------------- Y: makoon: data recieve
    let post_resualt = http_request.post("http://localhost:3000/send_data").body(body_in_res).send();
    println!("{:#?}", post_resualt.ok().unwrap().text().unwrap()); // B: text() -> gets the data

    // seting User-Agent like fields in Request ----------------------------------------------------------------------  Y: makoon: useragent in logs
    let post_header = http_request.post("http://localhost:3000/send_data").header("User-Agent", "Not Allways Fucking Chrome").send();
    println!("{:#?}", post_header.ok().unwrap().text().unwrap());

    // redirect with makoon ------------------------------------------------------------------------------------------ Y: makoon: redirection endpoints
    let redirection_policy_limit = Policy::limited(5);
    let http_client = ClientBuilder::new().redirect(redirection_policy_limit).build().ok().unwrap();
    let http_client_resualt = http_client.get("http://localhost:3000/weather").send();
    println!("{:#?}", http_client_resualt.ok().unwrap().text().unwrap());
}
