import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

import { AudioLoaderService } from '../audio-loader.service';
import { RippleDirective } from '../ripple.directive';
import { Subscription } from 'rxjs';

//----------------------------------------------------------------

class TextMeasure {
	static readonly canvas = document.createElement('canvas');
	static readonly context = this.canvas.getContext('2d')!;

	static widthOf(text: string, element: HTMLElement): number {
		TextMeasure.context.font = getComputedStyle(element).getPropertyValue('font');
		return TextMeasure.context.measureText(text).width;
	}
}

//----------------------------------------------------------------
// Backend types

type LearningTask = {
	word_id: number;
	sentence_id: number;
	word: string;
	word_pos: number;
	sentence: string;
	translations: string[];
};

type FinishedTask = {
	word_id: number;
	sentence_id: number;
	result: string;
};

//----------------------------------------------------------------

export enum TaskState {
	InputWord,
	Feedback,
};

enum TaskResult {
	Failed,
	Succeeded,
}

//----------------------------------------------------------------

@Component({
	selector: 'app-learn',
	standalone: true,
	imports: [CommonModule, RippleDirective, RouterModule],
	templateUrl: './learn.component.html',
	styleUrls: ['./learn.component.scss']
})
export class LearnComponent implements AfterViewInit {
	private taskState = TaskState.InputWord;
	currentTask?: LearningTask;
	preInputText = '';
	postInputText = '';
	buttonText = '';
	hint = '';

	private newAudioDataSubscription = this.audioLoader.newAudioData$.subscribe(() => this.changeDetector.detectChanges());

	@ViewChild('wordInput') 
	private wordInput!: ElementRef<HTMLInputElement>;

	constructor(public audioLoader: AudioLoaderService, private changeDetector: ChangeDetectorRef) {
		appWindow.setTitle('Gurskaft - learn');
	}

	ngAfterViewInit(): void {
		this.nextTask();
	}

	ngOnDestroy(): void {
		this.newAudioDataSubscription.unsubscribe();
		this.audioLoader.stopLoading();
	}

	isFeedback(): boolean {
		return this.taskState == TaskState.Feedback;
	}

	continue(): void {
		if (!this.currentTask) {
			return;
		}

		switch (this.taskState) {
			case TaskState.InputWord:
				if (this.wordInput.nativeElement.value.toLowerCase() == this.currentTask.word.toLowerCase()) {
					this.finishTask(TaskResult.Succeeded);
					this.showSuccessFeedback();
				}
				else {
					this.finishTask(TaskResult.Failed);
					this.retry();
				}
				this.playAudio();
				break;
			case TaskState.Feedback:
				this.nextTask();
				break;
		}

		this.wordInput.nativeElement.focus();
	}
	
	private nextTask(): void {
		invoke<LearningTask>('next_task').then(task => {
			this.preInputText = task.sentence.substring(0, task.word_pos);
			this.postInputText = task.sentence.substring(task.word_pos + task.word.length);

			const wordInput = this.wordInput.nativeElement;

			const wordWidth = TextMeasure.widthOf(task.word, wordInput);
			wordInput.style.width = `${wordWidth}px`;
			wordInput.value = '';
			wordInput.readOnly = false;
			wordInput.style.color = 'oklch(var(--on-surface))';
			
			this.hint = '';
			this.buttonText = 'Check';
			this.currentTask = task;
			this.taskState = TaskState.InputWord;
			
			this.audioLoader.newSentence(task.sentence, task.sentence_id);
			
			this.changeDetector.detectChanges();
		});
	}

	private finishTask(result: TaskResult): void {
		if (!this.currentTask) {
			return;
		}

		const task: FinishedTask = {
			word_id: this.currentTask.word_id,
			sentence_id: this.currentTask.sentence_id,
			result: result == TaskResult.Failed ? "Failed" : "Succeeded"
		};

		invoke('finish_task', { task });
	}

	private showSuccessFeedback(): void {
		const input = this.wordInput.nativeElement;
		input.style.color = 'oklch(var(--good))';
		input.readOnly = true;
		this.buttonText = 'Next';
		this.taskState = TaskState.Feedback;
		this.changeDetector.detectChanges();
	}

	private retry(): void {
		if (!this.currentTask) {
			return;
		}

		let hint = this.currentTask.word;
		let pos = 0;
		for (const letter of this.wordInput.nativeElement.value.substring(0, hint.length)) {
			const pos_in_hint = hint.indexOf(letter, pos);
			if (pos_in_hint >= 0) {
				hint = `${hint.substring(0, pos_in_hint)}<span class="good">${letter}</span>${hint.substring(pos_in_hint + 1)}`;
				pos = pos_in_hint + 1 + `<span class="good"></span>`.length;
			}
			else {
				pos++;
			}
		}
		this.hint = hint;
		this.wordInput.nativeElement.value = "";
		this.changeDetector.detectChanges();
	}

	handleInputKeyUp(event: KeyboardEvent): void {
		if (event.key == 'Enter') {
			this.continue();
		}
	}

	playAudio(): void {
		this.audioLoader.play();
	}
}
