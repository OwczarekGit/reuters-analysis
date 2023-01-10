import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'chart-icon',
  templateUrl: './chart-icon.component.html',
  styleUrls: ['./chart-icon.component.css']
})
export class ChartIconComponent implements OnInit {

  @Input('active')
  active: boolean = false

  constructor() { }

  ngOnInit(): void {
  }

}
