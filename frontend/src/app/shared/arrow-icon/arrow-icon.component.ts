import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'arrow-icon',
  templateUrl: './arrow-icon.component.html',
  styleUrls: ['./arrow-icon.component.css']
})
export class ArrowIconComponent implements OnInit {

  @Input("active")
  active: boolean = false;

  @Input("direction")
  direction: string = "left";

  @Input("bottomLine")
  bottomLine: boolean = false;

  constructor() { }

  ngOnInit(): void {
  }

}
