// Author:      machinageist
// Date:        2026-05-16
// Description: Handlers under /.well-known. Currently serves the RFC 9116
//              security.txt advertising the security contact and policy.
//
// Notes:       The Expires field is REQUIRED by RFC 9116 and is hardcoded.
//              Renew it by updating SECURITY_TXT_EXPIRES at least once a year.
//              Returning text/plain explicitly because the default ServeDir
//              MIME inference could produce something unexpected for files
//              under a dot-directory.

use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::IntoResponse;

// Renew yearly. RFC 9116 §2.5.5 requires a single Expires value.
const SECURITY_TXT_EXPIRES: &str = "2027-05-16T00:00:00Z";

// Return the canonical security.txt body
pub async fn security_txt() -> impl IntoResponse {
    // Body is built at request time so the expires constant is the single source of truth
    let body = format!(
        "Contact: mailto:machinageist@proton.me\n\
         Expires: {SECURITY_TXT_EXPIRES}\n\
         Preferred-Languages: en\n\
         Canonical: https://machinageist.dev/.well-known/security.txt\n"
    );

    let mut headers = HeaderMap::new();
    // text/plain is the only correct content type per RFC 9116 §3
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    // Allow CDN to cache for an hour — body only changes on yearly renewal
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
    );

    (headers, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn security_txt_contains_required_fields() {
        let response = security_txt().await.into_response();
        assert_eq!(response.status(), 200);
        let ct = response
            .headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(ct.starts_with("text/plain"));

        let body_bytes = to_bytes(response.into_body(), 4096).await.unwrap();
        let body = std::str::from_utf8(&body_bytes).unwrap();
        assert!(body.contains("Contact: mailto:machinageist@proton.me"));
        assert!(body.contains("Expires:"));
        assert!(body.contains("Canonical: https://machinageist.dev/.well-known/security.txt"));
    }
}
