use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};

// Middleware function signature for Axum:
//   request: the incoming HTTP request
//   next: the rest of the middleware chain + the handler
// Pattern: receive request → pass inward → get response back → modify → return
// The request flows inward through middleware layers.
// The response flows back outward through the same layers in reverse.
pub async fn add_security_headers(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    // Content-Security-Policy — the primary defense against XSS.
    // 'self' means: only load resources from the same origin as the page.
    // No CDN links, no external scripts, no inline event handlers.
    // frame-ancestors 'none' is the CSP equivalent of X-Frame-Options: DENY.
    // Red team context: a strict CSP makes XSS exploitation significantly harder —
    // the attacker can inject a script tag but the browser refuses to execute it.    
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

    // Strict-Transport-Security — tells the browser to always use HTTPS.
    // max-age=31536000 = one year, in seconds.
    // After the first visit, the browser enforces HTTPS itself without asking the server.
    // This defeats SSL stripping attacks where an attacker intercepts HTTP traffic
    // before it can be upgraded to HTTPS.
    // Harmless without TLS — becomes active the moment a certificate is deployed.
    headers.insert(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains"
        .parse().unwrap(),
    );
    
    // X-Content-Type-Options: nosniff — prevents MIME type sniffing.
    // Browsers normally try to guess content type if the server's declaration
    // seems wrong. An attacker can abuse this: upload a text file, the browser
    // sniffs it as JavaScript and executes it.
    // nosniff: trust the Content-Type header, never guess.
    headers.insert(
        "x-content-type-options",
        "nosniff".parse().unwrap(),
    );

    // X-Frame-Options: DENY — prevents this page from loading in an iframe.
    // Clickjacking attack: embed your site invisibly in an iframe on the attacker's page,
    // trick users into clicking buttons they can't see (OAuth confirmations, payments).
    // DENY means no framing at all — not even by the same origin.
    headers.insert(
        "x-frame-options",
        "DENY".parse().unwrap(),
    );

    // Referrer-Policy — controls what URL is sent in the Referer header.
    // strict-origin-when-cross-origin: sends full URL for same-origin requests,
    // only the origin (no path) for cross-origin requests.
    // Prevents leaking your URL structure (and any sensitive path parameters) to
    // third-party sites when users click external links.
    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // Permissions-Policy — explicitly disable browser features you don't use.
    // Empty () means: deny this feature entirely, even to injected scripts.
    // If an attacker injects code that tries to access the camera, the browser
    // refuses because the policy was set by the server before any injection occurred.
    headers.insert(
        "permissions-policy",
        "cameras=(), microphone=(), geolocation=(), paymenrr=()"
        .parse().unwrap(),
    );

    // Remove the Server header — don't advertise what software you're running.
    // Default behavior doesn't add one, but be explicit.
    // Removing it forces attackers to do active fingerprinting rather than
    // reading the version off your response headers.
    headers.remove("server");

    response
}


