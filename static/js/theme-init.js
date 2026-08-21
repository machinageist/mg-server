// Theme init — runs in <head> before first paint so there is no flash.
// Loaded as an external file because the site CSP is script-src 'self' (an inline
// script would be blocked). Stored mode is one of MODES; unset or unknown means
// system. 'system' resolves to the OS preference; every other mode is itself.
(function () {
    var MODES = ["system", "lunarcore", "solarcore", "ayu", "dark", "gruvbox", "material", "monokai", "nord", "one-dark", "one-light", "solarized", "tomorrow-night", "zenburn", "catppuccin-frappe", "catppuccin-latte", "catppuccin-macchiato", "catppuccin-mocha", "everforest", "kanagawa", "rose-pine", "tokyo-night", "amber", "crt", "matrix", "teletext", "nes", "c64", "gameboy", "cyberpunk", "synthwave", "tron", "vaporwave", "blueprint", "cloud", "dawn", "light", "paper", "sepia"];
    try {
        var mode = localStorage.getItem("theme");
        if (MODES.indexOf(mode) < 0) mode = "system";
        var resolved = mode === "system"
            ? (matchMedia("(prefers-color-scheme: light)").matches ? "solarcore" : "lunarcore")
            : mode;
        document.documentElement.setAttribute("data-theme", resolved);
    } catch (e) {}
})();
