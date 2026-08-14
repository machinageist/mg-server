// Model for the /labs progress page. The route is not built yet, so nothing
// constructs a Lab outside the tests — but the tests must run, because the
// anti-overclaim guards in here are cited by criteria.md as an enforcement
// point and were dead for as long as the module was undeclared.
#[allow(dead_code)]
pub mod lab;
pub mod markdown;
pub mod page;
pub mod post;
pub mod project;
