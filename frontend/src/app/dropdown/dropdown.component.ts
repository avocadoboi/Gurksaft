import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';

export interface DropdownOption {
	value: string;
	text: string;
}

@Component({
	selector: 'app-dropdown',
	standalone: true,
	imports: [CommonModule, DropdownOptionComponent],
	templateUrl: './dropdown.component.html',
	styleUrls: ['./dropdown.component.scss']
})
export class DropdownComponent {
	@Input() placeholder: string = "";
	options: DropdownOption[] = [];
	selectedIndex = -1; // -1 before selection
}
