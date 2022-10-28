
const invoke = (window as any).__TAURI__.invoke;
// const emit = (window as any).__TAURI__.event.emit;
const listen = (window as any).__TAURI__.event.listen;

type Language = {
	name: string,
	id: string,
};

invoke("get_language_list", {}).then((languages: Language[]) => {
	for (const dropdown of document.getElementsByClassName("language-dropdown")) {
		for (const language of languages) {
			const option = document.createElement("option");
			option.value = language.name;
			option.innerText = language.name;
			dropdown.appendChild(option);
		}
	}
});

//----------------------------------------------------------------

function add_language_to_dropdown(language_dropdown: HTMLSelectElement, language: string): void {
	if (!language) {
		return;
	}
	
	const new_option = document.createElement("option");
	new_option.value = language;
	new_option.innerText = language;

	const options = language_dropdown.options;
	options.add(new_option, [...options].find(option => option.value > language));
}
function remove_language_from_dropdown(language_dropdown: HTMLSelectElement, language: string): void {
	if (!language) {
		return;
	}
	
	[...language_dropdown.options].find(option => option.value == language)?.remove();
}

//----------------------------------------------------------------

const translation_languages_element = document.getElementById("translation-language-list")! as HTMLUListElement;
const translation_language_dropdown = document.getElementById("translation-language")! as HTMLSelectElement;

document.getElementById("add-translation-language-button")?.addEventListener("click", () => {
	if (translation_language_dropdown.selectedIndex <= 0) {
		return;
	}

	const translation_language = translation_language_dropdown.value;
	
	translation_languages_element.innerHTML += `
		<div>
			<p>${translation_language}</p>
			<button class="red-button remove-translation-language-button">
				<svg viewBox="0 0 48 48"><path fill="currentColor" d="M9.25 26.3v-4.55h29.5v4.55Z"/></svg>
			</button>
		</div>
	`;

	translation_language_dropdown.options.remove(translation_language_dropdown.selectedIndex);
	translation_language_dropdown.selectedIndex = 0;

	remove_language_from_dropdown(target_language_dropdown, translation_language);

	update_download_button();

	for (const element of translation_languages_element.children) {
		element.lastElementChild?.addEventListener("click", () => {
			const language = element.firstElementChild?.innerHTML!;
			add_language_to_dropdown(translation_language_dropdown, language);
			add_language_to_dropdown(target_language_dropdown, language);
			element.remove();
			update_download_button();
		});
	}
});

//----------------------------------------------------------------

const target_language_dropdown = document.getElementById("target-language-dropdown")! as HTMLSelectElement;
let previous_target_language = "";

target_language_dropdown.addEventListener("change", () => {
	add_language_to_dropdown(translation_language_dropdown, previous_target_language);
	previous_target_language = target_language_dropdown.value;
	remove_language_from_dropdown(translation_language_dropdown, previous_target_language);
	update_download_button();
});

//----------------------------------------------------------------

const download_button = document.getElementById("download-button")! as HTMLButtonElement;

function update_download_button() {
	download_button.disabled = target_language_dropdown.selectedIndex <= 0 || translation_languages_element.childElementCount <= 0;
}

//----------------------------------------------------------------

download_button.addEventListener("click", () => {
	document.getElementById("add-language")!.style.display = "none";
	document.getElementById("download-progress")!.style.display = "flex";

	const translation_languages: string[] = [];
	for (const element of translation_languages_element.children) {
		translation_languages.push(element.firstElementChild?.innerHTML!);
	}
	
	invoke("download_language_data", {
		info: {
			target_language: target_language_dropdown.value,
			translation_languages
		}
	});
});

//----------------------------------------------------------------

const download_status_element = document.getElementById("download-progress-text")! as HTMLElement;

const progress_to_string = (progress: number) => 
	progress <= 1 ? `${Math.round(progress*100)}%` : `${progress} bytes`;

listen("download_status", (event: any) => {
	if (event.payload.DownloadingWords) {
		let status = event.payload.DownloadingWords;
		download_status_element.innerText = `Downloading word list... ${progress_to_string(status.progress)}`;
	}
	else if (event.payload.PreparingSentenceFile) {
		let status = event.payload.PreparingSentenceFile;
		download_status_element.innerText = `Preparing ${status.translation_language} translations...`;
	}
	else if (event.payload.DownlodingSentenceFile) {
		let status = event.payload.DownlodingSentenceFile;
		download_status_element.innerText = `Downloading ${status.translation_language} translations... ${progress_to_string(status.progress)}`;
	}
	else if (event.payload.Loading) {
		download_status_element.innerText = "Parsing data...";
	}
});
