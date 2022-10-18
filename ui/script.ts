
class TextMeasure {
	static readonly canvas = document.createElement("canvas");
	static readonly context = this.canvas.getContext("2d")!;

	static width_of(text: string, element: HTMLElement): number {
		TextMeasure.context.font = getComputedStyle(element).getPropertyValue("font");
		return TextMeasure.context.measureText(text).width;
	}
}

//----------------------------------------------------------------

type LearningTask = {
	word_id: number,
	sentence_id: number,
	word: string,
	word_pos: number,
	sentence: string,
	translations: string[],
};

let current_task: LearningTask | null = null;

enum TaskState {
	InputWord,
	Feedback,
}

let task_state = TaskState.InputWord;

//----------------------------------------------------------------

const invoke = (window as any).__TAURI__.invoke;

const word_input = document.getElementById("word_input")! as HTMLInputElement;
const next_button = document.getElementById("next_button")! as HTMLButtonElement;

function next_task() {
	const pre_input_word_text = document.getElementById("pre_input_word_text") as HTMLParagraphElement;
	const post_input_word_text = document.getElementById("post_input_word_text") as HTMLParagraphElement;
	const translations_list = document.getElementById("translations") as HTMLUListElement;

	invoke("next_task", {}).then((task: LearningTask) => {
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

enum TaskResult {
	Failed,
	Succeeded,
}

type FinishedTask = {
	word_id: number,
	sentence_id: number,
	result: string,
};

function finish_task(result: TaskResult) {
	if (current_task == null) {
		return;
	}
	
	let finished_task: FinishedTask = {
		word_id: current_task?.word_id,
		sentence_id: current_task?.sentence_id,
		result: result == TaskResult.Failed ? "Failed" : "Succeeded",
	};
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

//----------------------------------------------------------------

const task_page = document.getElementById("task")!;
const options_page = document.getElementById("options")!;

document.getElementById("options_button")?.addEventListener("click", () => {
	task_page.style.display = "none";
	options_page.style.display = "flex";
});

document.getElementById("back_button")?.addEventListener("click", () => {
	task_page.style.display = "flex";
	options_page.style.display = "none";
});

//----------------------------------------------------------------

type WeightFactors = {
	succeeded: number,
	failed: number,
};

const success_weight_factor_input = document.getElementById("success_weight_factor_input")! as HTMLInputElement;
const failure_weight_factor_input = document.getElementById("failure_weight_factor_input")! as HTMLInputElement;

invoke("get_weight_factors", {}).then((weight_factors: WeightFactors) => {
	console.log(weight_factors);
	success_weight_factor_input.value = weight_factors.succeeded.toString();
	failure_weight_factor_input.value = weight_factors.failed.toString();
});
