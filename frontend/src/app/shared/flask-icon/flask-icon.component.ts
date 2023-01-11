import {AfterViewInit, Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'flask-icon',
  templateUrl: './flask-icon.component.html',
  styleUrls: ['./flask-icon.component.css']
})
export class FlaskIconComponent implements OnInit, AfterViewInit {
  @Input('active')
  active: boolean = false

  @Input("fill")
  fill!: number
  constructor() { }

  ngOnInit(): void {
  }

  ngAfterViewInit(): void {
  }

}
