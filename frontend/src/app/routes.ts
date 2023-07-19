import { Routes } from '@angular/router';
import { AddLanguageComponent } from './add-language/add-language.component';
import { DownloadLanguageDataComponent } from './download-language-data/download-language-data.component';

export const routeConfig: Routes = [
	{
		path: 'add-language',
		component: AddLanguageComponent
	},
	{
		path: 'download-language-data',
		component: DownloadLanguageDataComponent
	}
];