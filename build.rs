// Author:      machinageist
// Date:        2026-07-12
// Description: Build script that stamps a compile-time UTC build timestamp into
//              the binary as the BUILD_TS environment variable. The status/vitals
//              readout reads it at runtime so the footer can show when this exact
//              process image was built.
// Notes:       Emits epoch seconds rather than a formatted string so no calendar
//              math (or an extra build-dependency) is needed here — chrono formats
//              it at runtime. With no rerun-if-* directives, cargo reruns this
//              script whenever the package changes, so the stamp tracks rebuilds.

use std::time::{SystemTime, UNIX_EPOCH};

// Emit the build timestamp as epoch seconds
fn main() {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=BUILD_TS={seconds}");
}
