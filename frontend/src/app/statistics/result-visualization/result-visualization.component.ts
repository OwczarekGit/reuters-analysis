import {Component, Input, OnInit} from '@angular/core';
import {Result} from "../../shared/result";
import {ChartType, Column} from "angular-google-charts";

@Component({
  selector: 'app-result-visualization',
  templateUrl: './result-visualization.component.html',
  styleUrls: ['./result-visualization.component.css']
})
export class ResultVisualizationComponent implements OnInit {

  @Input("result")
  result!: Result


  chartType: ChartType = ChartType.Bar

  @Input("ready")
  ready: boolean = false;

  rows: [string, number][] = [];
  ratioChartType: ChartType = ChartType.PieChart;

  constructor() {
    this.getRows();
  }

  ngOnInit(): void {
  }

  getRows(): any {
   this.rows =  [['5', 5],['2', 2]];
  }

  getColumns(): Column[] {
    return  []
  }

}
