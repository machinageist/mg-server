// Drill timer — shows how long a drill set has taken, counted from the first
// answer typed. Loaded at the end of the drill page only. The timer element is
// hidden in the HTML and shown here, so without JavaScript the page has no
// timer and loses nothing else. The time is never sent to the server and does
// not affect grading.
(function () {
    var timer = document.querySelector("[data-drill-timer]");
    var form = document.querySelector(".study-form");
    if (!timer || !form) {
        return;
    }
    var output = timer.querySelector("[data-drill-elapsed]");
    var started = null;

    // Write the elapsed time as minutes and seconds
    function tick() {
        var seconds = Math.floor((Date.now() - started) / 1000);
        var rest = seconds % 60;
        output.textContent = Math.floor(seconds / 60) + ":" + (rest < 10 ? "0" : "") + rest;
    }

    // Start counting on the first keystroke in any answer field
    form.addEventListener("input", function () {
        if (started !== null) {
            return;
        }
        started = Date.now();
        timer.hidden = false;
        tick();
        setInterval(tick, 1000);
    });
})();
