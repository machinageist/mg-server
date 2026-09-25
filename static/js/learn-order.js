// Learn sidebar order — which of the three page orderings the wiki sidebar shows.
// Loaded in <head> so a stored choice applies before first paint, the same way
// theme-init.js does for the theme. The server renders all three orderings and
// CSS shows the one named by data-learn-order on <html>. Without JavaScript the
// toggle stays hidden and the topic order shows.
(function () {
    var ORDERS = ["topic", "ccna", "netplus"];
    var KEY = "learn-order";
    var root = document.documentElement;

    // Read the stored order, falling back to topic
    function stored() {
        try {
            var order = localStorage.getItem(KEY);
            return ORDERS.indexOf(order) >= 0 ? order : "topic";
        } catch (e) {
            return "topic";
        }
    }

    // Show an order and mark its button as pressed
    function apply(order) {
        root.setAttribute("data-learn-order", order);
        var buttons = document.querySelectorAll(".wiki-order [data-order]");
        for (var i = 0; i < buttons.length; i++) {
            var on = buttons[i].getAttribute("data-order") === order;
            buttons[i].setAttribute("aria-pressed", on ? "true" : "false");
        }
    }

    apply(stored());

    // Reveal the toggle and persist each choice once the sidebar exists
    document.addEventListener("DOMContentLoaded", function () {
        var group = document.querySelector(".wiki-order");
        if (!group) return;
        group.hidden = false;
        apply(stored());
        group.addEventListener("click", function (e) {
            var button = e.target.closest("[data-order]");
            if (!button) return;
            var order = button.getAttribute("data-order");
            try { localStorage.setItem(KEY, order); } catch (err) {}
            apply(order);
        });
    });
})();
