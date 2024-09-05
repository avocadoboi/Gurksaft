import { AfterViewInit, ChangeDetectorRef, Component, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';

import { DropdownComponent } from '../dropdown/dropdown.component';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';
import { NumberInputDirective } from '../number-input.directive';
import { RippleDirective } from '../ripple.directive';

//----------------------------------------------------------------

class Range {
	min = 0;
	max = 1;
};

class WordMemoryParameters {
	easy_threshold = 0;
	learned_threshold = 0;
	change_rate_range = new Range();
	change_rate_half_time = 0;
	initial_memory = 0;
}

class WeightFactors {
	succeeded = 0;
	failed = 0;
}

class Options {
	current_language = "";
	saved_languages: string[] = [];
	weight_factors = new WeightFactors();
	word_memory_parameters = new WordMemoryParameters();
}

@Component({
	selector: 'app-options',
	standalone: true,
	imports: [CommonModule, FormsModule, RouterModule, 
		DropdownComponent, DropdownOptionComponent, NumberInputDirective, RippleDirective],
	templateUrl: './options.component.html',
	styleUrls: ['./options.component.scss']
})
export class OptionsComponent implements AfterViewInit {
	@ViewChild('languageDropdown') 
	private languageDropdown!: DropdownComponent;
	options = new Options();

	constructor(private changeDetector: ChangeDetectorRef) {
		appWindow.setTitle('Gurksaft - options');
	}
	
	ngAfterViewInit(): void {
		// console
		invoke<Options>("get_options").then(options => {
			// console.log("Got em");
			this.options = options;
			this.changeDetector.detectChanges();
			this.languageDropdown.select(options.current_language);
			this.changeDetector.detectChanges();
		});
	}

	changeLanguage(option: DropdownOptionComponent): void {
		// invoke("set_current_language", { languageName: option.value });
	}

	saveWeightFactors(): void {
		// invoke("set_weight_factors", { factors: this.options.weightFactors });
	}
	saveWordMemoryParameters(): void {
		// invoke("set_word_memory_parameters", { parameters: this.options.wordMemoryParameters });
	}
}
