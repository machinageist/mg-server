use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};

pub async fn add_security_headers(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    headers.insert(
        "content-security-policy",
        "default-src 'self';        \
        script-src 'self';          \
        style-src 'self';           \
        img-src 'self' data:;       \
        font-src 'self';            \
        connect-src 'self;          \
        frame-ancestors 'none'"
        .parse().unwrap(),
    );

    headers.insert(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains"
        .parse().unwrap(),
    );
    
    headers.insert(
        "x-content-type-options",
        "nosniff".parse().unwrap(),
    );

    headers.insert(
        "x-frame-options",
        "DENY".parse().unwrap(),
    );

    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    headers.insert(
        "permissions-policy",
        "cameras=(), microphone=(), geolocation=(), paymenrr=()"
        .parse().unwrap(),
    );

    headers.remove("server");

    response
}


