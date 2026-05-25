// Theme toggle — flip data-theme on <html> and remember the choice.
// The initial theme is applied by the inline script in base.html <head>
// before paint, so this only handles user clicks afterward.
(function () {
    var btn = document.getElementById("theme-toggle");
    if (!btn) return;

    btn.addEventListener("click", function () {
        var current = document.documentElement.getAttribute("data-theme");
        var next = current === "light" ? "dark" : "light";
        document.documentElement.setAttribute("data-theme", next);
        try {
            localStorage.setItem("theme", next);
        } catch (e) {}
    });
})();
