import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RippleDirective } from '../ripple.directive';
import { invoke } from '@tauri-apps/api';

import { DropdownComponent } from '../dropdown/dropdown.component';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';

@Component({
	selector: 'app-add-language',
	standalone: true,
	imports: [CommonModule, DropdownComponent, DropdownOptionComponent, RippleDirective],
	templateUrl: './add-language.component.html',
	styleUrls: ['./add-language.component.scss']
})
export class AddLanguageComponent implements AfterViewInit {
	targetLanguages: string[] = [];
	translationLanguages: string[] = [];
	
	ngAfterViewInit(): void {
		invoke<string[]>('get_language_list', {}).then(languages => {
			this.targetLanguages = this.translationLanguages = languages;
		});
	}
}
