import { ChangeDetectorRef, Component, NgZone } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';

import { listen, UnlistenFn } from '@tauri-apps/api/event';
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
	unlisten?: UnlistenFn;
	
	constructor(private router: Router, private changeDetector: ChangeDetectorRef, private zone: NgZone) { 
		appWindow.setTitle('Gurksaft - downloading data');
		
		const progress_to_string = (progress: number) => 
			progress <= 1 ? `${Math.round(progress*100)}%` : `${progress} bytes`;
	
		listen<any>("download_status", event => {
			if (event.payload.DownloadingWords) {
				const status = event.payload.DownloadingWords;
				this.statusMessage = `Downloading word list... ${progress_to_string(status.progress)}`;
			}
			else if (event.payload.PreparingSentenceFile) {
				const status = event.payload.PreparingSentenceFile;
				this.statusMessage = `Preparing ${status.translation_language} translations...\nThis might take a while.`;
			}
			else if (event.payload.DownloadingSentenceFile) {
				const status = event.payload.DownloadingSentenceFile;
				this.statusMessage = `Downloading ${status.translation_language} translations... ${progress_to_string(status.progress)}`;
			}
			else if (event.payload.DownloadingVoiceModel) {
				const status = event.payload.DownloadingVoiceModel;
				this.statusMessage = `Downloading voice model ${status.index + 1}/${status.total}... ${progress_to_string(status.progress)}`;
			}
			else if (event.payload == 'Loading') {
				this.statusMessage = "Parsing data...";
			}
			else if (event.payload == 'Finished') {
				if (this.unlisten) {
					this.unlisten();
				}
				zone.run(() => router.navigate(['learn']));
			}
			else {
				console.error(JSON.stringify(event));
			}
			changeDetector.detectChanges();
		}).then(unlisten => this.unlisten = unlisten);
	}
}
