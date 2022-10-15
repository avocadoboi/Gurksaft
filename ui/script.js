"use strict";
class TextMeasure {
    static canvas = document.createElement("canvas");
    static context = this.canvas.getContext("2d");
    static width_of(text, element) {
        TextMeasure.context.font = getComputedStyle(element).getPropertyValue("font");
        return TextMeasure.context.measureText(text).width;
    }
}
let current_task = null;
var TaskState;
(function (TaskState) {
    TaskState[TaskState["InputWord"] = 0] = "InputWord";
    TaskState[TaskState["Feedback"] = 1] = "Feedback";
})(TaskState || (TaskState = {}));
let task_state = TaskState.InputWord;
//----------------------------------------------------------------
const invoke = window.__TAURI__.invoke;
const word_input = document.getElementById("word_input");
const next_button = document.getElementById("next_button");
function next_task() {
    const pre_input_word_text = document.getElementById("pre_input_word_text");
    const post_input_word_text = document.getElementById("post_input_word_text");
    const translations_list = document.getElementById("translations");
    invoke("next_task", {}).then((task) => {
        pre_input_word_text.innerText = task.sentence.substring(0, task.word_pos);
        post_input_word_text.innerText = task.sentence.substring(task.word_pos + task.word.length);
        const word_width = TextMeasure.width_of(task.word, word_input);
        word_input.style.width = `${word_width + 3}px`;
        word_input.value = "";
        word_input.placeholder = "";
        word_input.readOnly = false;
        word_input.style.color = "white";
        next_button.innerText = "Check";
        current_task = task;
        task_state = TaskState.InputWord;
        translations_list.innerHTML = "";
        for (const sentence of task.translations) {
            const item = document.createElement("li");
            item.innerText = sentence;
            translations_list.appendChild(item);
        }
    });
}
document.fonts.ready.then(next_task);
//----------------------------------------------------------------
var TaskResult;
(function (TaskResult) {
    TaskResult[TaskResult["Failed"] = 0] = "Failed";
    TaskResult[TaskResult["Succeeded"] = 1] = "Succeeded";
})(TaskResult || (TaskResult = {}));
function finish_task(result) {
    if (current_task == null) {
        return;
    }
    let finished_task = {
        word_id: current_task?.word_id,
        sentence_id: current_task?.sentence_id,
        result: result == TaskResult.Failed ? "Failed" : "Succeeded",
    };
    console.log(finished_task);
    invoke("finish_task", { task: finished_task });
}
//----------------------------------------------------------------
function enter() {
    if (current_task == null) {
        return;
    }
    switch (task_state) {
        case TaskState.InputWord:
            if (word_input.value.toLowerCase() == current_task?.word.toLowerCase()) {
                finish_task(TaskResult.Succeeded);
                show_success_feedback();
            }
            else {
                finish_task(TaskResult.Failed);
                retry();
            }
            break;
        case TaskState.Feedback:
            next_task();
            break;
    }
    word_input.focus();
}
next_button.addEventListener("click", enter);
word_input.addEventListener("keyup", e => {
    if (e.key == "Enter") {
        enter();
    }
});
//----------------------------------------------------------------
function show_success_feedback() {
    word_input.style.color = "rgb(20, 255, 50)";
    word_input.readOnly = true;
    next_button.innerText = "Next";
    task_state = TaskState.Feedback;
}
function retry() {
    if (current_task == null) {
        return;
    }
    word_input.value = "";
    word_input.placeholder = current_task?.word;
}
