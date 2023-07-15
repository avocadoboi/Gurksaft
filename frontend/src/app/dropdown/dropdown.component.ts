import { 
	AfterContentInit,
	ElementRef, 
	ViewChild, 
	Component, 
	Input, 
	HostListener, 
	ContentChildren, 
	QueryList 
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';

import { animate, reciprocalEaseOutTransferFunction } from '../animation';
import { startWith } from 'rxjs';

@Component({
	selector: 'app-dropdown',
	standalone: true,
	imports: [CommonModule, DropdownOptionComponent],
	templateUrl: './dropdown.component.html',
	styleUrls: ['./dropdown.component.scss']
})
export class DropdownComponent implements AfterContentInit {
	@ViewChild('optionsView') optionsView!: ElementRef<HTMLDivElement>;
	@ViewChild('dropdownText') dropdownText!: ElementRef<HTMLDivElement>;
	@ContentChildren(DropdownOptionComponent) options!: QueryList<DropdownOptionComponent>;
	
	@Input() placeholder: string = '';

	selectedIndex = -1; // -1 before selection
	isOpen: boolean = false;

	// Used to tell whether a click is outside or inside the dropdown.
	// Outside always closes it while inside toggles it.
	wasClicked: boolean = false;

	toggle(event: MouseEvent) {
		this.isOpen = !this.isOpen;
		animate(t => {
			let factor = reciprocalEaseOutTransferFunction(t, 0.75);
			factor = this.isOpen ? factor : 1 - factor;
			
			const height = factor*250;
			this.optionsView.nativeElement.style.maxHeight = `${height}px`;

			this.optionsView.nativeElement.style.opacity = `${Math.min(factor*1.2, 1)}`;
		}, 200);
		this.wasClicked = true;
	}

	@HostListener('document:click', ['event']) clickOutside(event: MouseEvent) {
		if (this.isOpen && !this.wasClicked) {
			this.toggle(event);
		}
		this.wasClicked = false;
	}

	ngAfterContentInit() {
		this.options.changes.subscribe((options: QueryList<DropdownOptionComponent>) => {
			let index = 0;
			options.forEach((option: DropdownOptionComponent) => {
				option.dropdown = this;
				option.index = index++;
			});
		})
	}

	select(option: DropdownOptionComponent) {
		this.selectedIndex = option.index;
		this.dropdownText.nativeElement.innerText = option.text;
	}
}
