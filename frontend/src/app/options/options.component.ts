import { AfterViewInit, ChangeDetectorRef, Component, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';

import { RippleDirective } from '../ripple.directive';
import { DropdownComponent } from '../dropdown/dropdown.component';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';

//----------------------------------------------------------------

type WeightFactors = {
	succeeded: number;
	failed: number;
};

@Component({
	selector: 'app-options',
	standalone: true,
	imports: [CommonModule, DropdownComponent, DropdownOptionComponent, FormsModule, RippleDirective, RouterModule],
	templateUrl: './options.component.html',
	styleUrls: ['./options.component.scss']
})
export class OptionsComponent implements AfterViewInit {
	@ViewChild('languageDropdown') 
	private languageDropdown!: DropdownComponent;
	
	savedLanguages: string[] = [];
	weightFactors: WeightFactors = {
		succeeded: 1,
		failed: 1
	};

	constructor(private changeDetector: ChangeDetectorRef) {
		appWindow.setTitle('Gurksaft - options');

		invoke<WeightFactors>("get_weight_factors").then(weight_factors => this.weightFactors = weight_factors);
	}
	
	ngAfterViewInit(): void {
		console.log("Gettin languages!!!!");
		invoke<string[]>('get_saved_language_list').then(languages => {
			this.savedLanguages = languages;
			this.changeDetector.detectChanges();
	
			invoke<string>('get_current_language').then(language => {
				console.log(language);
				this.languageDropdown.select(language);
				this.changeDetector.detectChanges();
			});
		});
	}

	changeLanguage(option: DropdownOptionComponent): void {
		invoke("set_current_language", { languageName: option.value });
	}

	setSuccessWeight(): void {
		invoke("set_success_weight_factor", { factor: this.weightFactors.succeeded });
	}
	setFailureWeight(): void {
		invoke("set_failure_weight_factor", { factor: this.weightFactors.failed });
	}

}
