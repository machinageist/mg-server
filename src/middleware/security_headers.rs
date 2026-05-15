// Author:      machinageist
// Date:        2026-04
// Description: Middleware that stamps defensive HTTP headers onto every outgoing
//              response. Runs after the handler produces a response, before the
//              bytes leave the server. One function covers all routes because it
//              is applied at the router level, not per-handler.
//              Removes the Server header to avoid advertising software version.
//
// Notes:       Middleware flow — request passes inward through layers to handler,
//              response passes back outward. This function sits on the outbound
//              path so it sees every response regardless of which handler produced it.
//
//              Header purposes (blue team / red team):
//                CSP            — restricts script/style sources; defeats XSS injection
//                                 even if an attacker finds an injection vector
//                HSTS           — forces HTTPS in the browser; defeats SSL stripping
//                nosniff        — prevents MIME sniffing; closes file-upload-as-script vector
//                X-Frame-Options — prevents iframe embedding; defeats clickjacking
//                Referrer-Policy — limits URL leakage to external sites on link click
//                Permissions-Policy — denies camera/mic/geo to injected scripts
//                Server removal — forces active fingerprinting instead of passive reading

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

// Add all security headers to the response, remove Server disclosure header
pub async fn add_security_headers(request: Request<Body>, next: Next) -> Response<Body> {
    // Pass request inward — waits for handler to produce a response
    let mut response = next.run(request).await;
    let headers      = response.headers_mut();

    // -----------------------------------------------------------------------
    // Content-Security-Policy — restrict all resource loading to same origin
    // -----------------------------------------------------------------------
    // 'self' = only load from this domain — no CDNs, no inline scripts
    // frame-ancestors 'none' = CSP equivalent of X-Frame-Options: DENY
    headers.insert(
        "content-security-policy",
        "default-src 'self'; \
         script-src 'self'; \
         style-src 'self'; \
         img-src 'self' data:; \
         font-src 'self'; \
         connect-src 'self'; \
         frame-ancestors 'none'"
            .parse()
            .unwrap(),
    );

    // -----------------------------------------------------------------------
    // Strict-Transport-Security — enforce HTTPS for one year
    // -----------------------------------------------------------------------
    // Browser caches this — subsequent visits upgrade to HTTPS before any request is sent
    // includeSubDomains — applies to all subdomains as well
    headers.insert(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );

    // -----------------------------------------------------------------------
    // X-Content-Type-Options — disable MIME sniffing
    // -----------------------------------------------------------------------
    // Browser trusts declared Content-Type — never re-interprets a .txt as .js
    headers.insert("x-content-type-options", "nosniff".parse().unwrap());

    // -----------------------------------------------------------------------
    // X-Frame-Options — prevent iframe embedding
    // -----------------------------------------------------------------------
    // DENY = no framing allowed from any origin including same origin
    headers.insert("x-frame-options", "DENY".parse().unwrap());

    // -----------------------------------------------------------------------
    // Referrer-Policy — limit URL leakage on outbound links
    // -----------------------------------------------------------------------
    // Cross-origin: send only origin (no path). Same-origin: send full URL.
    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // -----------------------------------------------------------------------
    // Permissions-Policy — deny browser features this site does not use
    // -----------------------------------------------------------------------
    // Empty () = deny entirely — injected scripts cannot request these features
    headers.insert(
        "permissions-policy",
        "camera=(), microphone=(), geolocation=(), payment=()"
            .parse()
            .unwrap(),
    );

    // Remove Server header — do not advertise software name or version
    headers.remove("server");

    response
}
