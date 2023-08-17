import { AfterViewInit, Component, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';

import { appWindow } from '@tauri-apps/api/window';

import { RippleDirective } from '../ripple.directive';
import { WeightDistributionComponent } from '../weight-distribution/weight-distribution.component';
import { RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';

//----------------------------------------------------------------

export type LearningWord = {
	word: string,
	weight: number,
	long_term_memory: number,
};

export type WordData = {
	words: LearningWord[],
	max_weight: number,
};

@Component({
	selector: 'app-statistics',
	standalone: true,
	imports: [CommonModule, RouterModule, 
		RippleDirective, WeightDistributionComponent],
	templateUrl: './statistics.component.html',
	styleUrls: ['./statistics.component.scss']
})
export class StatisticsComponent implements AfterViewInit {
	@ViewChild('weightDistributionGraph')
	weightDistributionGraph!: WeightDistributionComponent;

	@ViewChild('longTermMemoryGraph')
	longTermMemoryGraph!: WeightDistributionComponent;
	
	constructor() {
		appWindow.setTitle('Gurskaft - statistics');
	}

	ngAfterViewInit(): void {
		invoke<WordData>('get_word_data').then(data => {
			this.weightDistributionGraph.setWordData(data, false);
			this.longTermMemoryGraph.setWordData(data, true);
		});
	}
}
