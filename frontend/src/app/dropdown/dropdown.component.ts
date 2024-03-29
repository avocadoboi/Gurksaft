import { 
	AfterContentInit,
	ElementRef, 
	ViewChild, 
	Component, 
	Input, 
	HostListener, 
	ContentChildren, 
	QueryList, 
	Output,
	EventEmitter
} from '@angular/core';
import { CommonModule } from '@angular/common';

import { DropdownOptionComponent } from '../dropdown-option/dropdown-option.component';
import { animate, reciprocalEaseOutTransferFunction } from '../common';
import { RippleDirective } from '../ripple.directive';

//----------------------------------------------------------------

@Component({
	selector: 'app-dropdown',
	standalone: true,
	imports: [CommonModule, DropdownOptionComponent, RippleDirective],
	templateUrl: './dropdown.component.html',
	styleUrls: ['./dropdown.component.scss']
})
export class DropdownComponent implements AfterContentInit {
	@ViewChild('optionsView') 
	private optionsView!: ElementRef<HTMLDivElement>;
	
	@ViewChild('dropdownText') 
	private dropdownText!: ElementRef<HTMLDivElement>;

	@ContentChildren(DropdownOptionComponent) 
	private options!: QueryList<DropdownOptionComponent>;
	
	@Input() 
	placeholder: string = '';

	@Output() 
	selectionChange = new EventEmitter<DropdownOptionComponent>();

	selectedOption?: DropdownOptionComponent;
	private isOpen: boolean = false;

	// Is used to tell whether a click is outside or inside the dropdown.
	// Outside always closes it while inside toggles it.
	private wasClicked: boolean = false;

	toggle(): void {
		this.isOpen = !this.isOpen;
		animate(t => {
			let factor = reciprocalEaseOutTransferFunction(t, 0.75);
			factor = this.isOpen ? factor : 1 - factor;
			
			const height = factor*200;
			this.optionsView.nativeElement.style.maxHeight = `${height}px`;

			this.optionsView.nativeElement.style.opacity = `${Math.min(factor*1.2, 1)}`;
		}, 200);
		this.wasClicked = true;
	}

	@HostListener('document:click') 
	private clickOutside(): void {
		if (this.isOpen && !this.wasClicked) {
			this.toggle();
		}
		this.wasClicked = false;
	}

	ngAfterContentInit(): void {
		this.options.changes.subscribe((options: QueryList<DropdownOptionComponent>) => {
			let index = 0;
			for (const option of options) {
				option.dropdown = this;
				option.index = index++;
			}
		});
	}

	select(option: DropdownOptionComponent | string): void {
		if (option instanceof DropdownOptionComponent) {
			this.selectedOption = option;
			this.dropdownText.nativeElement.innerText = option.text;
			this.dropdownText.nativeElement.classList.remove('placeholder');
			this.selectionChange.emit(option);
		}
		else {
			const foundOption = this.options.find(item => item.text == option);
			if (foundOption) {
				this.select(foundOption);
			}
			else {
				console.log('Heck');
			}
		}
	}

	removeSelection(): void {
		this.dropdownText.nativeElement.innerText = this.placeholder;
		this.dropdownText.nativeElement.classList.add('placeholder');
		this.selectedOption = undefined;
	}
}
