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
//----------------------------------------------------------------
function add_language_to_dropdown(language_dropdown, language) {
    if (!language) {
        return;
    }
    const new_option = document.createElement("option");
    new_option.value = language;
    new_option.innerText = language;
    const options = language_dropdown.options;
    options.add(new_option, [...options].find(option => option.value > language));
}
function remove_language_from_dropdown(language_dropdown, language) {
    if (!language) {
        return;
    }
    [...language_dropdown.options].find(option => option.value == language)?.remove();
}
//----------------------------------------------------------------
const translation_languages_element = document.getElementById("translation-language-list");
const translation_language_dropdown = document.getElementById("translation-language");
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
    for (const element of translation_languages_element.children) {
        element.lastElementChild?.addEventListener("click", () => {
            const language = element.firstElementChild?.innerHTML;
            add_language_to_dropdown(translation_language_dropdown, language);
            add_language_to_dropdown(target_language_dropdown, language);
            element.remove();
        });
    }
});
//----------------------------------------------------------------
const target_language_dropdown = document.getElementById("target-language-dropdown");
let previous_target_language = "";
target_language_dropdown.addEventListener("change", () => {
    add_language_to_dropdown(translation_language_dropdown, previous_target_language);
    previous_target_language = target_language_dropdown.value;
    remove_language_from_dropdown(translation_language_dropdown, previous_target_language);
});
//----------------------------------------------------------------
document.getElementById("download-button")?.addEventListener("click", () => {
    document.getElementById("add-language").style.display = "none";
    document.getElementById("download-progress").style.display = "flex";
});
