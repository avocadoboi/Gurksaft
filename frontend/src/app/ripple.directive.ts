import { Directive, ElementRef, HostListener, Input } from '@angular/core';

import { animate, reciprocalEaseOutTransferFunction } from './animation';

// We want to fade away the ripples no matter where the user releases the mouse button on the page.
// A ripple instance is removed the next time the user presses on the element with the ripple on it.
document.addEventListener('mouseup', () => {
    for (const ripple of document.getElementsByClassName('ripple-instance') as HTMLCollectionOf<HTMLElement>) {
        ripple.style.backgroundColor = 'transparent';
    }
});

class RippleInstance {
	private element: HTMLDivElement;

	constructor(parent: HTMLElement, event: MouseEvent, color: string) {
		this.element = document.createElement('div');
		this.element.className = 'ripple-instance';
		this.element.style.borderRadius = '50%';
		this.element.style.backgroundColor = color;
	
		const [x, y, width, height] = 
			[event.offsetX, event.offsetY, parent.clientWidth, parent.clientHeight];
		
		const radius = Math.sqrt(Math.pow(Math.max(x, width - x), 2) + Math.pow(Math.max(y, height - y), 2));
		this.element.style.width = `${radius*2}px`;
		this.element.style.height = `${radius*2}px`;
	
		this.element.style.left = `${x}px`;
		this.element.style.top = `${y}px`;

		parent.appendChild(this.element);

		animate(t => {
			const minScale = 0.2;
			const scale = minScale + (1 - minScale)*reciprocalEaseOutTransferFunction(t);
			this.element.style.transform = `translateX(-50%) translateY(-50%) scale(${scale})`;
		}, 650);
	}
	
	remove() {
		this.element.remove();
	}
}

function getCssColor(name: string, alpha: number = 1): string {
	return `rgb(${getComputedStyle(document.documentElement).getPropertyValue(name)}, ${alpha})`;
}

@Directive({
	selector: '[appRipple]',
	standalone: true
})
export class RippleDirective {
	// Ripple type (see getColor definition)
	@Input() appRipple = '';
	
	private element: HTMLElement;
	private overlay: HTMLDivElement;
	private rippleInstance: RippleInstance | null = null;
	
	constructor(private elementReference: ElementRef<HTMLElement>) {
		this.element = elementReference.nativeElement;
		this.element.style.overflow = 'hidden';
		this.element.style.position = 'relative';

		this.overlay = document.createElement('div');
		this.overlay.className = 'hover-overlay';
	
		this.element.appendChild(this.overlay);
	}

	@HostListener('mouseenter') showHoverOverlay() {
		this.overlay.style.backgroundColor = this.getColor();
	}
	@HostListener('mouseleave') hideHoverOverlay() {
		this.overlay.style.backgroundColor = 'transparent';
	}

	@HostListener('mousedown', ['$event']) addRipple(event: MouseEvent) {
		this.rippleInstance?.remove();
		this.rippleInstance = new RippleInstance(this.element, event, this.getColor());
	}
	
	getColor(): string {
		if (this.appRipple == 'light') {
			return getCssColor('--highlight');
		}
		if (this.appRipple == 'accent') {
			return getCssColor('--accent');
		}
		if (this.appRipple == 'secondary') {
			return getCssColor('--secondary');
		}
		if (this.appRipple == 'bad') {
			return getCssColor('--bad');
		}
		return 'black';
	}
}
