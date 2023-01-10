import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'k-icon',
  templateUrl: './k-icon.component.html',
  styleUrls: ['./k-icon.component.css']
})
export class KIconComponent implements OnInit {

  @Input('active')
  active: boolean = false
  constructor() { }

  ngOnInit(): void {
  }

}
