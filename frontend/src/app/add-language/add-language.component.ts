import { AfterViewInit, Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';

import { invoke } from '@tauri-apps/api';

import { DropdownComponent } from '../dropdown/dropdown.component';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';
import { RippleDirective } from '../ripple.directive';

// Could do a binary search but there is no reason to for this use case (less than 1k elements).
function insertSortedArray<T>(array: T[], toInsert: T): void {
	const insertIndex = array.findIndex(element => element > toInsert);
	if (insertIndex == -1) {
		array.push(toInsert);
	}
	else {
		array.splice(insertIndex, 0, toInsert);
	}
}

@Component({
	selector: 'app-add-language',
	standalone: true,
	imports: [CommonModule, DropdownComponent, DropdownOptionComponent, RippleDirective],
	templateUrl: './add-language.component.html',
	styleUrls: ['./add-language.component.scss']
})
export class AddLanguageComponent implements AfterViewInit {
	languageOptions: string[] = [];
	
	targetLanguage: string = '';
	translationLanguages: string[] = [];
	
	constructor(private router: Router) {}
	
	ngAfterViewInit(): void {
		// Populate the dropdowns with language options.
		invoke<string[]>('get_language_list', {}).then(languages => {
			this.languageOptions = languages;
		});
	}

	selectTargetLanguage(option: DropdownOptionComponent): void {
		// Remove the newly selected target language from the language dropdowns.
		this.languageOptions.splice(option.index, 1);

		if (this.targetLanguage != '') {
			// Insert previously selected target language into the language dropdowns.
			insertSortedArray(this.languageOptions, this.targetLanguage);
		}
		
		this.targetLanguage = option.value;
	}
	selectTranslationLanguage(option: DropdownOptionComponent): void {
		this.translationLanguages.push(option.value);
		this.languageOptions.splice(option.index, 1);
		option.dropdown.removeSelection();
	}
	removeTranslationLanguage(event: MouseEvent): void {
		const translationLanguageElement = (event.target as HTMLElement).parentElement;
		const otherElements = translationLanguageElement?.parentElement?.children;
		if (otherElements) {
			const index = [...otherElements].indexOf(translationLanguageElement);
			insertSortedArray(this.languageOptions, this.translationLanguages[index]);
			this.translationLanguages.splice(index, 1);
		}
	}
	download(): void {
		invoke('download_language_data', {
			info: {
				target_language: this.targetLanguage,
				translation_languages: this.translationLanguages
			}
		});
		this.router.navigate(['download-language-data']);
	}
}
