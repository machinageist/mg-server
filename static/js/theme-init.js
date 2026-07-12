// Theme init — runs in <head> before first paint so there is no flash.
// Loaded as an external file because the site CSP is script-src 'self' (an inline
// script would be blocked). Stored mode is 'system' | 'light' | 'dark'; unset means
// system. 'system' resolves to the OS preference; the choice sticks across pages.
(function () {
    try {
        var mode = localStorage.getItem("theme");
        if (mode !== "light" && mode !== "dark") mode = "system";
        var resolved = mode === "system"
            ? (matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark")
            : mode;
        document.documentElement.setAttribute("data-theme", resolved);
    } catch (e) {}
})();
