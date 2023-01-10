import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'accuracy-icon',
  templateUrl: './accuracy-icon.component.html',
  styleUrls: ['./accuracy-icon.component.css']
})
export class AccuracyIconComponent implements OnInit {

  @Input("showBorder")
  showBorder: boolean = false;
  @Input("active")
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
