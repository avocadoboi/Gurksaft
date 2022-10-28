"use strict";
function overlay_color_for_element(element) {
    if (element.classList.contains("overlay-light")) {
        return "white";
    }
    if (element.classList.contains("overlay-accent")) {
        return getComputedStyle(element).getPropertyValue("--accent");
    }
    if (element.classList.contains("overlay-secondary")) {
        return getComputedStyle(element).getPropertyValue("--secondary");
    }
    if (element.classList.contains("overlay-red")) {
        return getComputedStyle(element).getPropertyValue("--red");
    }
    return "black";
}
/*
    Using css animations with a cubic bezier easing for animating the ripples doesn't work on webkitgtk
    for some reason (the animation is always linear), so here's a hardcoded animation.
*/
function update_ripple(time_stamp, element) {
    let ripple_start_time = element.ripple_start_time;
    if (ripple_start_time == undefined) {
        ripple_start_time = element.ripple_start_time = time_stamp;
    }
    const total_time = 500;
    const min_scale = 0.4;
    const t = Math.min(time_stamp - ripple_start_time, total_time);
    const scale = min_scale + (1 - min_scale) * (1 - Math.exp(-7 * t / total_time)) / (1 - Math.exp(-7));
    element.style.transform = `translateX(-50%) translateY(-50%) scale(${scale})`;
    if (t != total_time && element.parentElement) {
        requestAnimationFrame(time_stamp => update_ripple(time_stamp, element));
    }
}
function add_ripple(element, event) {
    const ripple = document.createElement("div");
    ripple.className = "ripple-element";
    ripple.style.position = "absolute";
    ripple.style.borderRadius = "50%";
    ripple.style.backgroundColor = overlay_color_for_element(element);
    const width = element.clientWidth;
    const height = element.clientHeight;
    const x = event.clientX - element.offsetLeft;
    const y = event.clientY - element.offsetTop;
    const radius = Math.sqrt(Math.pow(Math.max(x, width - x), 2) + Math.pow(Math.max(y, height - y), 2));
    ripple.style.width = `${radius * 2}px`;
    ripple.style.height = `${radius * 2}px`;
    ripple.style.left = `${x}px`;
    ripple.style.top = `${y}px`;
    for (const ripple of element.getElementsByClassName("ripple-element")) {
        ripple.remove();
    }
    element.appendChild(ripple);
    update_ripple(performance.now(), ripple);
}
function fade_ripple(element) {
    for (const ripple of element.getElementsByClassName("ripple-element")) {
        ripple.style.backgroundColor = "transparent";
    }
}
const elements_with_ripple = document.getElementsByClassName("ripple");
for (const element of elements_with_ripple) {
    element.style.overflow = "hidden";
    element.style.position = "relative";
    const overlay = document.createElement("div");
    overlay.className = "hover-overlay";
    element.addEventListener("mouseenter", () => overlay.style.backgroundColor = overlay_color_for_element(element));
    element.addEventListener("mouseleave", () => overlay.style.backgroundColor = "transparent");
    element.addEventListener("mousedown", event => add_ripple(element, event));
    document.addEventListener("mouseup", () => fade_ripple(element));
    element.appendChild(overlay);
}
