"use strict";
const invoke = window.__TAURI__.invoke;
invoke("get_language_list", {}).then((languages) => {
    for (const dropdown of document.getElementsByClassName("language-dropdown")) {
        for (const language of languages) {
            const option = document.createElement("option");
            option.value = language.name;
            option.innerText = language.name;
            dropdown.appendChild(option);
        }
    }
});
const translation_languages = [];
const translation_languages_element = document.getElementById("translation-language-list");
const translation_language_dropdown = document.getElementById("translation-language");
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
