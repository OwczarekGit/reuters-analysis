import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'empty-icon',
  templateUrl: './empty-icon.component.html',
  styleUrls: ['./empty-icon.component.css']
})
export class EmptyIconComponent implements OnInit {


  @Input('active')
  active: boolean = false

  constructor() { }

  ngOnInit(): void {
  }

}
