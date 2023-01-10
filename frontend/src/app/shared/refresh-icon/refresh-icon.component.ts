import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'refresh-icon',
  templateUrl: './refresh-icon.component.html',
  styleUrls: ['./refresh-icon.component.css']
})
export class RefreshIconComponent implements OnInit {
  @Input('active')
  active: boolean = false

  constructor() { }

  ngOnInit(): void {
  }

}
