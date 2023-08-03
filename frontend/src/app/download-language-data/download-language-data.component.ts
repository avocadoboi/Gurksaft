import { ChangeDetectorRef, Component, Input, NgZone, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';

import { event } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';

@Component({
	selector: 'app-download-language-data',
	standalone: true,
	imports: [CommonModule],
	templateUrl: './download-language-data.component.html',
	styleUrls: ['./download-language-data.component.scss']
})
export class DownloadLanguageDataComponent {
	statusMessage: string = 'Loading...';
	
	constructor(private router: Router, private changeDetector: ChangeDetectorRef, private zone: NgZone) { 
		appWindow.setTitle('Gurksaft - downloading data');
		
		const progress_to_string = (progress: number) => 
			progress <= 1 ? `${Math.round(progress*100)}%` : `${progress} bytes`;
	
		event.listen("download_status", (event: any) => {
			if (event.payload.DownloadingWords) {
				const status = event.payload.DownloadingWords;
				this.statusMessage = `Downloading word list... ${progress_to_string(status.progress)}`;
			}
			else if (event.payload.PreparingSentenceFile) {
				const status = event.payload.PreparingSentenceFile;
				this.statusMessage = `Preparing ${status.translation_language} translations...\nThis might take a while.`;
			}
			else if (event.payload.DownlodingSentenceFile) {
				const status = event.payload.DownlodingSentenceFile;
				this.statusMessage = `Downloading ${status.translation_language} translations... ${progress_to_string(status.progress)}`;
			}
			else if (event.payload == 'Loading') {
				this.statusMessage = "Parsing data...";
			}
			else if (event.payload == 'Finished') {
				zone.run(() => router.navigate(['learn']));
			}
			else {
				console.error(JSON.stringify(event));
			}
			changeDetector.detectChanges();
		});
	}
}
