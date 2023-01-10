import {Component, ElementRef, EventEmitter, Input, OnDestroy, OnInit, Output, ViewChild} from '@angular/core';
import {Result} from "../../../shared/result";

@Component({
  selector: 'app-result-list-entry',
  templateUrl: './result-list-entry.component.html',
  styleUrls: ['./result-list-entry.component.css']
})
export class ResultListEntryComponent implements OnInit, OnDestroy {

  @Input("result")
  result!: Result;

  @Output("visualize")
  visualizeEventEmitter: EventEmitter<any> = new EventEmitter<any>();

  @Output("delete")
  deleteEventEmitter: EventEmitter<any> = new EventEmitter<any>();

  @ViewChild("box")
  box!: ElementRef<HTMLElement>

  constructor() { }

  ngOnInit(): void {
  }

  delete() {
    this.box.nativeElement.style.transform = 'translateX(100%) rotateZ(45deg)';
    setTimeout(()=> this.deleteEventEmitter.emit(),500);
  }
  visualize() {
    this.visualizeEventEmitter.emit();
  }

  ngOnDestroy(): void {
  }

}
