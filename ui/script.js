"use strict";
const pre_input_word_text = document.getElementById("pre_input_word_text");
const word_input = document.getElementById("word_input");
const post_input_word_text = document.getElementById("post_input_word_text");
const translations_list = document.getElementById("translations");
const invoke = window.__TAURI__.invoke;
class TextMeasure {
    static canvas = document.createElement("canvas");
    static context = this.canvas.getContext("2d");
    static width_of(text, element) {
        TextMeasure.context.font = getComputedStyle(element).getPropertyValue("font");
        return TextMeasure.context.measureText(text).width;
    }
}
let current_task = null;
function next_task() {
    invoke("next_task", {}).then((task) => {
        pre_input_word_text.innerText = task.sentence.substring(0, task.word_pos);
        post_input_word_text.innerText = task.sentence.substring(task.word_pos + task.word.length);
        const word_width = TextMeasure.width_of(task.word, word_input);
        word_input.style.width = `${word_width + 3}px`;
        word_input.value = "";
        current_task = task;
        translations_list.innerHTML = "";
        for (const sentence of task.translations) {
            const item = document.createElement("li");
            item.innerText = sentence;
            translations_list.appendChild(item);
        }
    });
}
document.fonts.ready.then(next_task);
