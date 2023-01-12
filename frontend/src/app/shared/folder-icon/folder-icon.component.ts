import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'folder-icon',
  templateUrl: './folder-icon.component.html',
  styleUrls: ['./folder-icon.component.css']
})
export class FolderIconComponent implements OnInit {

  @Input("active")
  active: boolean = false;
  constructor() { }

  ngOnInit(): void {
  }

}
