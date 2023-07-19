import { bootstrapApplication } from "@angular/platform-browser";
import { provideRouter, withComponentInputBinding } from '@angular/router';

import { AppComponent } from "./app/app.component";
import { routeConfig } from './app/routes';

bootstrapApplication(AppComponent, { providers: [provideRouter(routeConfig, withComponentInputBinding())] })
	.catch(error => console.error(error));
