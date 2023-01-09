import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import {ExecutionModule} from "./execution/execution.module";
import {StatisticsModule} from "./statistics/statistics.module";
import {Result} from "./shared/result";

@NgModule({
  declarations: [
    AppComponent
  ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        ExecutionModule,
        StatisticsModule
    ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {


  getResult(result:Result) {

  }

}
