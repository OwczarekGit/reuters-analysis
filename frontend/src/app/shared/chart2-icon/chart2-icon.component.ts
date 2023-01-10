import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'chart2-icon',
  templateUrl: './chart2-icon.component.html',
  styleUrls: ['./chart2-icon.component.css']
})
export class Chart2IconComponent implements OnInit {

  @Input('active')
  active: boolean = false

  constructor() { }

  ngOnInit(): void {
  }

}
