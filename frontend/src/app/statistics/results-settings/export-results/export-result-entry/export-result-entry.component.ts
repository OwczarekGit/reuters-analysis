import {Component, ElementRef, EventEmitter, Input, OnInit, Output, ViewChild} from '@angular/core';
import {Result} from "../../../../shared/result";

@Component({
  selector: 'app-export-result-entry',
  templateUrl: './export-result-entry.component.html',
  styleUrls: ['./export-result-entry.component.css']
})
export class ExportResultEntryComponent {
  @Input("result")
  result!: Result;

  @ViewChild("box")
  box!: ElementRef<HTMLElement>
  @Input("active")
  active: boolean = false;

  constructor() { }

}
