import { Component } from '@angular/core';
import { AddLanguageComponent } from './add-language/add-language.component';
import { RouterModule } from '@angular/router';

@Component({
	selector: 'app-root',
	standalone: true,
	imports: [AddLanguageComponent, RouterModule],
	templateUrl: 'app.component.html',
	styles: [':host { display: contents; }']
})
export class AppComponent {
}
