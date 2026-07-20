// Theme init — runs in <head> before first paint so there is no flash.
// Loaded as an external file because the site CSP is script-src 'self' (an inline
// script would be blocked). Stored mode is one of MODES; unset or unknown means
// system. 'system' resolves to the OS preference; every other mode is itself.
(function () {
    var MODES = ["system", "lunarcore", "solarcore", "dark", "light", "crt", "amber", "paper", "dawn", "cloud", "gameboy", "c64", "teletext", "nes", "matrix", "solarized", "nord", "gruvbox", "synthwave", "vaporwave", "cyberpunk", "tron", "blueprint", "sepia"];
    try {
        var mode = localStorage.getItem("theme");
        if (MODES.indexOf(mode) < 0) mode = "system";
        var resolved = mode === "system"
            ? (matchMedia("(prefers-color-scheme: light)").matches ? "solarcore" : "lunarcore")
            : mode;
        document.documentElement.setAttribute("data-theme", resolved);
    } catch (e) {}
})();
