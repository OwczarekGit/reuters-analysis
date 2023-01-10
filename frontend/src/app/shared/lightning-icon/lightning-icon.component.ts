import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'lightning-icon',
  templateUrl: './lightning-icon.component.html',
  styleUrls: ['./lightning-icon.component.css']
})
export class LightningIconComponent implements OnInit {
  @Input('active')
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
