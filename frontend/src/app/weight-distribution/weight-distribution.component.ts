import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';

import { getPaletteColor } from '../common';
import { LearningWord, WordData } from '../statistics/statistics.component';

//----------------------------------------------------------------

const barWidth = 35;
const barSpacing = 3;
const wordSpace = 140;
const textAngle = 80*Math.PI/180;

@Component({
	selector: 'app-weight-distribution',
	standalone: true,
	imports: [CommonModule],
	templateUrl: './weight-distribution.component.html',
	styleUrls: ['./weight-distribution.component.scss']
})
export class WeightDistributionComponent implements AfterViewInit {
	@ViewChild('canvas')
	private canvas!: ElementRef<HTMLCanvasElement>;
	private context!: CanvasRenderingContext2D;

	private wordData!: WordData;
	private isLongTermMemory: boolean = false;
	private scrollOffset: number = 0;
	private width: number = 0;
	private height: number = 0;

	setWordData(data: WordData, isLongTermMemory: boolean) {
		this.wordData = data;
		this.isLongTermMemory = isLongTermMemory;
		this.draw();
	}
	// constructor(private changeDetector: ChangeDetectorRef) {
	// 	changeDetector.detectChanges();
	// }
	
	ngAfterViewInit(): void {		
		const canvasElement = this.canvas.nativeElement;
		this.width = canvasElement.width;
		this.height = canvasElement.height;
		
		this.context = canvasElement.getContext('2d')!;
		this.context.font = `25px ${getComputedStyle(document.documentElement).getPropertyValue('font-family')}`;
		this.context.textAlign = 'start';
		this.context.textBaseline = 'middle';

		window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => this.draw());
	}

	draw(): void {
		this.context.clearRect(0, 0, this.width, this.height);

		const firstWordIndex = Math.max(0, Math.floor(this.scrollOffset / (barWidth + barSpacing)));
		const lastWordIndex = Math.min(this.wordData.words.length - 1, Math.floor((this.scrollOffset + this.width) / (barWidth + barSpacing)));
		
		this.drawBars(firstWordIndex, lastWordIndex);
		this.drawWords(firstWordIndex, lastWordIndex);
	}

	drawBars(firstWordIndex: number, lastWordIndex: number): void {
		this.context.fillStyle = getPaletteColor('primary');
		this.context.beginPath();
		for (let i = firstWordIndex; i <= lastWordIndex; i++) {
			const barHeight = (this.isLongTermMemory ? this.wordData.words[i].long_term_memory 
				: this.wordData.words[i].weight / this.wordData.max_weight) * (this.height - wordSpace);
			this.context.roundRect(
				i*(barWidth + barSpacing) - this.scrollOffset, 
				this.height - wordSpace - barHeight, 
				barWidth, barHeight, 
				Math.min(barWidth/2, barHeight/2)
			);
		}
		this.context.fill();
	}
	drawWords(firstWordIndex: number, lastWordIndex: number): void {
		const textOffsetDirection = {
			x: Math.cos(textAngle),
			y: -Math.sin(textAngle)
		};
		this.context.fillStyle = getPaletteColor('on-surface');
		this.context.save();
		this.context.translate(barWidth/2 - this.scrollOffset, this.height - wordSpace + 14);
		this.context.rotate(textAngle);
		for (let i = firstWordIndex; i <= lastWordIndex; i++) {
			const offset = i*(barWidth + barSpacing);
			this.context.fillText(this.wordData.words[i].word, offset*textOffsetDirection.x, offset*textOffsetDirection.y);
		}
		this.context.restore();
	}

	scroll(event: WheelEvent): void {
		this.scrollOffset = Math.min(this.wordData.words.length*(barWidth + barSpacing), Math.max(0, this.scrollOffset + event.deltaY));
		this.draw();
	}
}
