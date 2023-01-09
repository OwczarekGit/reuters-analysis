import {Component, Input, OnInit} from '@angular/core';
import {Result} from "../../shared/result";

@Component({
  selector: 'app-multiple-result-visualization',
  templateUrl: './multiple-result-visualization.component.html',
  styleUrls: ['./multiple-result-visualization.component.css']
})
export class MultipleResultVisualizationComponent implements OnInit {


  @Input("results")
  results!: Result[];
  constructor() { }

  ngOnInit(): void {
  }

}
