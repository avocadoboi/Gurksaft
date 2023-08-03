import { AfterViewInit, Component, ElementRef, Input, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RippleDirective } from '../ripple.directive';
import { DropdownComponent } from '../dropdown/dropdown.component';

@Component({
	selector: 'app-dropdown-option',
	standalone: true,
	imports: [CommonModule, RippleDirective],
	templateUrl: './dropdown-option.component.html',
	styleUrls: ['./dropdown-option.component.scss']
})
export class DropdownOptionComponent implements AfterViewInit {
	@Input() 
	value: string = '';
	
	@ViewChild('content') 
	private content!: ElementRef;
	text: string = '';

	index: number = 0;
	dropdown!: DropdownComponent;

	ngAfterViewInit() {
		if (this.content) {
			this.text = (this.content.nativeElement.textContent as string).trim();
		}
	}

	select() {
		if (this.dropdown) {
			this.dropdown.select(this);
		}
	}
}
