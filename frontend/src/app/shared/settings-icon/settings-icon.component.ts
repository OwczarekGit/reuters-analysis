import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'settings-icon',
  templateUrl: './settings-icon.component.html',
  styleUrls: ['./settings-icon.component.css']
})
export class SettingsIconComponent implements OnInit {
  @Input("active")
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
