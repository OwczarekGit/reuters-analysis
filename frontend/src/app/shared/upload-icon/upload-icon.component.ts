import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'upload-icon',
  templateUrl: './upload-icon.component.html',
  styleUrls: ['./upload-icon.component.css']
})
export class UploadIconComponent implements OnInit {

  @Input("active")
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
