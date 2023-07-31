import { AfterViewInit, ChangeDetectorRef, Component, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';

import { RippleDirective } from '../ripple.directive';
import { DropdownComponent } from '../dropdown/dropdown.component';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';

@Component({
	selector: 'app-options',
	standalone: true,
	imports: [CommonModule, DropdownComponent, DropdownOptionComponent, RippleDirective, RouterModule],
	templateUrl: './options.component.html',
	styleUrls: ['./options.component.scss']
})
export class OptionsComponent implements AfterViewInit {
	savedLanguages: string[] = [];
	@ViewChild('languageDropdown') languageDropdown!: DropdownComponent;

	constructor(private changeDetector: ChangeDetectorRef) {
		appWindow.setTitle('Gurksaft - options');
	}
	
	ngAfterViewInit(): void {
		invoke<string[]>('get_saved_language_list', {}).then(languages => {
			this.savedLanguages = languages;
			this.changeDetector.detectChanges();
	
			invoke<string>('get_current_language', {}).then(language => {
				console.log(language);
				this.languageDropdown.select(language);
			});
		});
	}
}
