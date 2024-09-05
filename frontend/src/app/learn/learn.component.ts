import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, QueryList, ViewChildren } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';

import { AudioLoaderService } from '../audio-loader.service';
import { RippleDirective } from '../ripple.directive';
import { FormsModule } from '@angular/forms';

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

type TaskWord = {
	id: number,
	word: string,
	position: number,	
};

type LearningTask = {
	sentence_id: number,
	sentence: string,
	translations: string[],
	review_words: TaskWord[],
};

enum WordReviewResult {
	Succeeded = "Succeeded",
	Failed = "Failed",
}

type FinishedWordReview = {
	word_id: number,
	result: WordReviewResult,
};

type FinishedTask = {
	word_reviews: FinishedWordReview[]
};

//----------------------------------------------------------------

class WordInput {
	inputText = '';
	word: string;
	width: number = 0;
	hint = '';
	finished = false;
	textAfter: string = '';
	
	index: number;
	wordId: number;

	constructor(word: TaskWord, textAfter: string, index: number) {
		this.word = word.word;
		this.width = TextMeasure.widthOf(word.word, document.getElementById('original-text')!);
		this.textAfter = textAfter;
		this.wordId = word.id;
		this.index = index;
	}
}

enum TaskState {
	Input,
	Feedback,
}

@Component({
	selector: 'app-learn',
	standalone: true,
	imports: [CommonModule, FormsModule, RouterModule, 
		RippleDirective],
	templateUrl: './learn.component.html',
	styleUrls: ['./learn.component.scss']
})
export class LearnComponent implements AfterViewInit {
	private taskState = TaskState.Input;
	
	preInputText = '';
	wordInputs: WordInput[] = [];
	
	buttonText = '';

	translations: string[] = [];

	@ViewChildren('wordInput')
	inputElements!: QueryList<ElementRef<HTMLInputElement>>;

	private newAudioDataSubscription = this.audioLoader.newAudioData$.subscribe(() => this.changeDetector.detectChanges());

	constructor(public audioLoader: AudioLoaderService, private changeDetector: ChangeDetectorRef) {
		appWindow.setTitle('Gurskaft - learn');
	}

	ngAfterViewInit(): void {
		this.nextTask();
		this.inputElements.changes.subscribe(() => {
			this.inputElements.first.nativeElement.focus();
		})
	}

	ngOnDestroy(): void {
		this.newAudioDataSubscription.unsubscribe();
		this.audioLoader.stopLoading();
	}

	isFeedback(): boolean {
		return this.taskState == TaskState.Feedback;
	}

	continue(): void {
		if (!this.wordInputs.length) {
			return;
		}

		switch (this.taskState) {
			case TaskState.Input:
				this.finishTask();
				this.playAudio();
				break;
			case TaskState.Feedback:
				this.nextTask();
				break;
		}

		// this.wordInput.nativeElement.focus();
	}
	
	private nextTask(): void {
		invoke<LearningTask>('next_task').then(task => {
			this.wordInputs = [];
			
			this.preInputText = task.sentence.substring(0, task.review_words[0].position);
			
			// The words are ordered by sentence position.
			for (let i = 0; i < task.review_words.length; i++) {
				this.wordInputs.push(new WordInput(
					task.review_words[i], 
					task.sentence.substring(
						task.review_words[i].position + task.review_words[i].word.length, 
						task.review_words.at(i + 1)?.position
					),
					i
				));
			}
			this.buttonText = 'Check';
			this.taskState = TaskState.Input;
			this.translations = task.translations;
			
			this.audioLoader.newSentence(task.sentence, task.sentence_id);
			
			this.changeDetector.detectChanges();
		});
	}

	private finishTask(): void {
		const task: FinishedTask = { word_reviews: [] };
		let areAllFinished = true;

		for (const input of this.wordInputs.filter(value => !value.finished)) {
			const succeeded = input.word == input.inputText;
			task.word_reviews.push({
				result: succeeded ? WordReviewResult.Succeeded : WordReviewResult.Failed,
				word_id: input.wordId
			});

			if (succeeded) {
				input.finished = true;
			}
			else {
				this.retryWord(input);
				areAllFinished = false;
			}
		}

		if (task.word_reviews.length) {
			invoke('finish_task', { task });
		}

		if (areAllFinished) {
			this.buttonText = 'Next';
			this.taskState = TaskState.Feedback;
		}
		else {
			const element = this.findFirstEditableWordInput(0);
			if (element) {
				element.focus();
			}
		}
	}

	private retryWord(wordInput: WordInput) {
		let hint = wordInput.word;
		let pos = 0;
		for (const letter of wordInput.inputText.substring(0, hint.length)) {
			const pos_in_hint = hint.indexOf(letter, pos);
			if (pos_in_hint >= 0) {
				hint = `${hint.substring(0, pos_in_hint)}<span class="good">${letter}</span>${hint.substring(pos_in_hint + 1)}`;
				pos = pos_in_hint + 1 + `<span class="good"></span>`.length;
			}
			else {
				pos++;
			}
		}
		wordInput.hint = hint;
		wordInput.inputText = '';
	}

	handleInputKeyUp(event: KeyboardEvent): void {
		const element = event.target as HTMLInputElement;
		let index = Number(element.dataset['index']);
		if (event.key == 'Enter') {
			this.continue();
		}
		else if (event.key == 'Backspace' && !this.wordInputs[index].inputText.length && index > 0) {
			const element = this.findLastEditableWordInput(index - 1);
			if (element) {
				element.focus();
			}
		}
		else if (event.key != 'ArrowRight' && element.value.length == element.maxLength && index < this.inputElements.length - 1) {
			const element = this.findFirstEditableWordInput(index + 1);
			if (element) {
				element.focus();
			}
		}
	}

	private findLastEditableWordInput(index: number): HTMLInputElement | undefined {
		while (true) {
			const element = this.inputElements.get(index)?.nativeElement!;
			if (element.readOnly) {
				if (index > 0) {
					index--;
					continue;
				}
				else {
					return undefined;
				}
			}
			return element;
		}

	}
	private findFirstEditableWordInput(index: number): HTMLInputElement | undefined {
		while (true) {
			const element = this.inputElements.get(index)?.nativeElement!;
			if (element.readOnly) {
				if (index < this.inputElements.length - 1) {
					index++;
					continue;
				}
				else {
					return undefined;
				}
			}
			return element;
		}
	}

	playAudio(): void {
		this.audioLoader.play();
	}
}
