import { Routes } from '@angular/router';
import { AddLanguageComponent } from './add-language/add-language.component';
import { DownloadLanguageDataComponent } from './download-language-data/download-language-data.component';
import { LearnComponent } from './learn/learn.component';
import { OptionsComponent } from './options/options.component';

export const routeConfig: Routes = [
	{
		path: 'add-language',
		component: AddLanguageComponent
	},
	{
		path: 'download-language-data',
		component: DownloadLanguageDataComponent
	},
	{
		path: 'learn',
		component: LearnComponent
	},
	{
		path: 'options',
		component: OptionsComponent
	}
];