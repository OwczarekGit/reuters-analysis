import { Component } from '@angular/core';
import {Result} from "./shared/result";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  result!: Result
  ready: boolean = false;
  results!: Result[];

  setResult(result: Result) {
    console.log("xd");
    this.result = result;
    setTimeout(
      () => {this.ready = true;},
      20
    );

  }
}
