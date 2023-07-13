import { Directive, ElementRef, HostListener, Input } from '@angular/core';

// We want to fade away the ripples no matter where the user releases the mouse button on the page.
// A ripple instance is removed the next time the user presses on the element with the ripple on it.
document.addEventListener('mouseup', () => {
    for (const ripple of document.getElementsByClassName('ripple-instance') as HTMLCollectionOf<HTMLElement>) {
        ripple.style.backgroundColor = "transparent";
    }
});

class RippleInstance {
	private element: HTMLDivElement;
	private startTime: DOMHighResTimeStamp | null = null;

	constructor(parent: HTMLElement, event: MouseEvent, color: string) {
		this.element = document.createElement('div');
		this.element.className = 'ripple-instance';
		this.element.style.borderRadius = '50%';
		this.element.style.backgroundColor = color;
	
		const width = parent.clientWidth;
		const height = parent.clientHeight;
		const x = event.pageX - parent.offsetLeft;
		const y = event.pageY - parent.offsetTop;
		
		const radius = Math.sqrt(Math.pow(Math.max(x, width - x), 2) + Math.pow(Math.max(y, height - y), 2));
		this.element.style.width = `${radius*2}px`;
		this.element.style.height = `${radius*2}px`;
	
		this.element.style.left = `${x}px`;
		this.element.style.top = `${y}px`;

		parent.appendChild(this.element);

		this.update(performance.now());
	}

	/*
		Using css animations with a cubic bezier easing for animating the ripples doesn't work on webkitgtk 
		for some reason (the animation is always linear), so here's a hardcoded animation.
	*/
	update(timeStamp: DOMHighResTimeStamp) {
		if (this.startTime == null) {
			this.startTime = timeStamp;
		}
	
		const totalTime = 700;
		const minScale = 0.3;
		const sharpness = 0.8;
	
		const t = Math.min(timeStamp - this.startTime, totalTime)/totalTime;
		const scale = minScale + (1 - minScale)*t/(t - Math.pow(1 - sharpness, 2)*(t - 1));
		this.element.style.transform = `translateX(-50%) translateY(-50%) scale(${scale})`;
	
		if (t < 1 && this.element.parentElement) {
			requestAnimationFrame(timeStamp => this.update(timeStamp));
		}
	}
	
	remove() {
		this.element.remove();
	}
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
			return 'white';
		}
		if (this.appRipple == 'accent') {
			return getComputedStyle(this.element).getPropertyValue('--accent');
		}
		if (this.appRipple == 'secondary') {
			return getComputedStyle(this.element).getPropertyValue('--secondary');
		}
		if (this.appRipple == 'red') {
			return getComputedStyle(this.element).getPropertyValue('--red');
		}
		return 'black';
	}
}
