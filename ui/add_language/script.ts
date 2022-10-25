
const invoke = (window as any).__TAURI__.invoke;

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

const translation_languages_element = document.getElementById("translation-language-list")! as HTMLUListElement;
const translation_language_dropdown = document.getElementById("translation-language")! as HTMLSelectElement;

document.getElementById("add-translation-language-button")?.addEventListener("click", () => {
	if (translation_language_dropdown.selectedIndex <= 0) {
		return;
	}
	translation_languages_element.innerHTML += `
		<div>
			<p>${translation_language_dropdown.value}</p>
			<button class="red-button remove-translation-language-button">
				<svg viewBox="0 0 48 48"><path fill="currentColor" d="M9.25 26.3v-4.55h29.5v4.55Z"/></svg>
			</button>
		</div>
	`;
	translation_language_dropdown.options.remove(translation_language_dropdown.selectedIndex);
	translation_language_dropdown.selectedIndex = 0;
});

const target_language_dropdown = document.getElementById("target-language-dropdown")! as HTMLSelectElement;

target_language_dropdown.addEventListener("change", () => {
	
});
