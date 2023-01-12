import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'floppy-icon',
  templateUrl: './floppy-icon.component.html',
  styleUrls: ['./floppy-icon.component.css']
})
export class FloppyIconComponent implements OnInit {

  @Input("active")
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
