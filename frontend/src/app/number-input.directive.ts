import { Directive, ElementRef } from '@angular/core';
import { NgModel } from '@angular/forms';

@Directive({
	selector: 'input[type="number"]',
	standalone: true
})
export class NumberInputDirective {
	constructor(private element: ElementRef, private ngModel: NgModel) {
		const input = element.nativeElement as HTMLInputElement;
		// ngModel.valueChanges?.subscribe(value => {
			// input.value = (Math.round(parseFloat(value)*1e3)/1e3).toString();
		// })
	}
}
