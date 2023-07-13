import { Component } from '@angular/core';
import { AddLanguageComponent } from './add-language/add-language.component';

@Component({
	selector: 'app-root',
	standalone: true,
	imports: [AddLanguageComponent],
	templateUrl: 'app.component.html',
})
export class AppComponent {
}
