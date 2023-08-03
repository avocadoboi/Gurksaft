import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

import { appWindow } from '@tauri-apps/api/window';

import { RippleDirective } from '../ripple.directive';
import { WeightDistributionComponent } from '../weight-distribution/weight-distribution.component';
import { RouterModule } from '@angular/router';

@Component({
	selector: 'app-statistics',
	standalone: true,
	imports: [CommonModule, RippleDirective, RouterModule, WeightDistributionComponent],
	templateUrl: './statistics.component.html',
	styleUrls: ['./statistics.component.scss']
})
export class StatisticsComponent {
	constructor() {
		appWindow.setTitle('Gurskaft - statistics');
	}
}
