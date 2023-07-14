import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
	selector: 'app-dropdown-option',
	standalone: true,
	imports: [CommonModule],
	templateUrl: './dropdown-option.component.html',
	styleUrls: ['./dropdown-option.component.scss']
})
export class DropdownOptionComponent {
	@Input() value!: string;
}
