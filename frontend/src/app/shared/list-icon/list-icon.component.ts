import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'list-icon',
  templateUrl: './list-icon.component.html',
  styleUrls: ['./list-icon.component.css']
})
export class ListIconComponent implements OnInit {

  @Input('active')
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
