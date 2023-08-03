import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';

import { invoke } from '@tauri-apps/api';

//----------------------------------------------------------------

type LearningWord = {
	word: string;
	weight: string;
}

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

	private words: LearningWord[] = [];

	ngAfterViewInit(): void {
		this.context = this.canvas.nativeElement.getContext('2d')!;

		invoke<LearningWord[]>('get_weights').then(words => {
			this.words = words;
			this.draw();
		});
	}

	draw(): void {
		const barWidth = 30;

		this.context.fillStyle = '#ddd';
		this.context.fillRect(0, 0, this.canvas.nativeElement.width, this.canvas.nativeElement.height);
	}
}
