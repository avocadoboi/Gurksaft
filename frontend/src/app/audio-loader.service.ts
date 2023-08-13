import { ChangeDetectorRef, Injectable } from '@angular/core';

import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { Subject } from 'rxjs';

export class SentenceAudio {
	private buffer!: AudioBuffer;

	constructor(private context: AudioContext, fileData: Uint8Array) {
		context.decodeAudioData(fileData.buffer, buffer => this.buffer = buffer);
	}

	play(): void {
		const source = this.context.createBufferSource();
		source.buffer = this.buffer;
		source.connect(this.context.destination);
		source.start();
	}
}

@Injectable({
	providedIn: 'root'
})
export class AudioLoaderService {
	private context = new AudioContext();
	clips: SentenceAudio[] = [];
	index = -1;
	// sentenceId = 0;
	private load_promise!: Promise<void>;
	private is_loading_done = true;

	private newAudioData = new Subject<void>();
	newAudioData$ = this.newAudioData.asObservable();

	constructor() {
		listen<number[]>('sentence_audio_data', data => {
			console.log('Got new audio clip!');
			this.clips.push(new SentenceAudio(this.context, Uint8Array.from(data.payload)));
			this.newAudioData.next();
		});
	}

	newSentence(sentence: string, id: number): void {
		const startLoading = () => {
			this.clips = [];
			this.index = -1;
			this.is_loading_done = false;
			this.load_promise = invoke('load_sentence_audio', { sentenceId: id, sentence }).then(_ => { this.is_loading_done = true; });
		};
		
		if (this.is_loading_done) {
			startLoading()
		}
		else {
			emit('cancel_sentence_audio');
			this.load_promise.finally(startLoading);
		}
	}
	stopLoading(): void {
		if (!this.is_loading_done) {
			emit('cancel_sentence_audio');
		}
	}
	play(): void {
		if (this.clips.length != 0) {
			this.index = (this.index + 1) % this.clips.length;
			this.clips[this.index].play();
		}
	}
	hasAudio(): boolean {
		return this.clips.length != 0;
	}
}
