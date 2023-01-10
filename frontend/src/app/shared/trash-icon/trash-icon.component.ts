import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'trash-icon',
  templateUrl: './trash-icon.component.html',
  styleUrls: ['./trash-icon.component.css']
})
export class TrashIconComponent implements OnInit {


  @Input('active')
  active: boolean = false
  constructor() { }

  ngOnInit(): void {
  }

}
