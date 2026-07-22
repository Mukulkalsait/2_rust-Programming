use axum::{
    Router,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use bytes::Bytes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Map endpoint routing path explicitly
    let app = Router::new().route("/calculator", post(handle_soap_add));

    // 2. Define a clean, full socket address boundary structure
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 SOAP/WSDL Service running at http://127.0.0");

    axum::serve(listener, app).await.unwrap();
}

async fn handle_soap_add(body: Bytes) -> impl IntoResponse {
    let xml_payload = String::from_utf8_lossy(&body);

    let a = extract_xml_tag(&xml_payload, "a").unwrap_or(0);
    let b = extract_xml_tag(&xml_payload, "b").unwrap_or(0);
    let sum = a + b;

    println!("📥 Received SOAP Request -> a: {}, b: {} | Result: {}", a, b, sum);

    let soap_response = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://xmlsoap.org" xmlns:calc="http://example.com">
   <soapenv:Header/>
   <soapenv:Body>
      <calc:AddResponse>
         <calc:result>{}</calc:result>
      </calc:AddResponse>
   </soapenv:Body>
</soapenv:Envelope>"#,
        sum
    );

    (StatusCode::OK, [(header::CONTENT_TYPE, "text/xml; charset=utf-8")], soap_response)
}

fn extract_xml_tag(xml: &str, tag: &str) -> Option<i32> {
    let open_pattern = format!(":{}", tag);
    let fallback_pattern = format!("<{}", tag);

    let start_idx = xml
        .find(&open_pattern)
        .map(|idx| idx + open_pattern.len() + 1)
        .or_else(|| xml.find(&fallback_pattern).map(|idx| idx + fallback_pattern.len() + 1))?;

    let close_pattern = format!("</");
    let end_idx = xml[start_idx..].find(&close_pattern)? + start_idx;

    xml[start_idx..end_idx].trim().parse::<i32>().ok()
}
