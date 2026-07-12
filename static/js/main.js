// Theme selector — a small System / Light / Dark menu in the header.
// theme-init.js (in <head>) applies the theme before paint; this file drives the
// selector, persists the choice, and keeps 'system' following the OS live.
(function () {
    var root = document.documentElement;
    var wrap = document.getElementById("theme-select");
    var btn = document.getElementById("theme-btn");
    var menu = document.getElementById("theme-menu");
    if (!wrap || !btn || !menu) return;

    var mql = matchMedia("(prefers-color-scheme: light)");
    var ICON = { system: "◐", light: "☀", dark: "☾" }; // ◐ ☀ ☾
    var items = menu.querySelectorAll("[data-mode]");

    // Read the stored mode (defaults to system)
    function getMode() {
        try { var m = localStorage.getItem("theme"); return (m === "light" || m === "dark") ? m : "system"; }
        catch (e) { return "system"; }
    }
    // Resolve a mode to an actual theme
    function resolve(mode) { return mode === "system" ? (mql.matches ? "light" : "dark") : mode; }

    // Apply + persist a chosen mode, then reflect it in the UI
    function apply(mode) {
        root.setAttribute("data-theme", resolve(mode));
        try { localStorage.setItem("theme", mode); } catch (e) {}
        render(mode);
    }
    // Reflect the current mode on the button and menu
    function render(mode) {
        var icon = btn.querySelector(".theme-icon");
        if (icon) icon.textContent = ICON[mode];
        btn.setAttribute("title", "Theme: " + mode + " (click to change)");
        items.forEach(function (b) {
            var on = b.getAttribute("data-mode") === mode;
            b.setAttribute("aria-checked", on ? "true" : "false");
            b.classList.toggle("is-current", on);
        });
    }

    // Open / close the menu
    function open() { menu.hidden = false; btn.setAttribute("aria-expanded", "true"); document.addEventListener("click", onDoc); document.addEventListener("keydown", onKey); }
    function close() { menu.hidden = true; btn.setAttribute("aria-expanded", "false"); document.removeEventListener("click", onDoc); document.removeEventListener("keydown", onKey); }
    function onDoc(e) { if (!wrap.contains(e.target)) close(); }
    function onKey(e) { if (e.key === "Escape") { close(); btn.focus(); } }

    btn.addEventListener("click", function () { menu.hidden ? open() : close(); });
    items.forEach(function (b) {
        b.addEventListener("click", function () { apply(b.getAttribute("data-mode")); close(); btn.focus(); });
    });
    // While in system mode, follow OS changes live
    mql.addEventListener("change", function () { if (getMode() === "system") root.setAttribute("data-theme", resolve("system")); });

    render(getMode());
})();
