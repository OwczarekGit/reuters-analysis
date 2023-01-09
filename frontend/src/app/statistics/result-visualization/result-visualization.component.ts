import {Component, Input, OnInit} from '@angular/core';
import {Result} from "../../shared/result";
import {ChartBase, ChartType, Column} from "angular-google-charts";
import {Observable} from "rxjs";

@Component({
  selector: 'app-result-visualization',
  templateUrl: './result-visualization.component.html',
  styleUrls: ['./result-visualization.component.css']
})
export class ResultVisualizationComponent implements OnInit {
  get result(): Result {
    return this._result;
  }

  @Input("result")
  set result(value: Result) {
    this._result = value;
    this.getRows();
    this.ready = true;
  }

  private _result!: Result


  options:any = {
    chart: {
      title: 'Classification objects',
      subtitle: 'Precision, Recall, and Fallout stats',

    },
    // Accepts also 'rgb(255, 0, 0)' format but not rgba(255, 0, 0, 0.2),
    // for that use fillOpacity versions
    // Colors only the chart area, simple version
    // chartArea: {
    //   backgroundColor: '#FF0000'
    // },
    // Colors only the chart area, with opacity
    chartArea: {
      backgroundColor: {
        fill: '#FF0000',
        fillOpacity: 0.1
      },
    },
    // Colors the entire chart area, simple version
    // backgroundColor: '#FF0000',
    // Colors the entire chart area, with opacity
    backgroundColor: "#aaa",
    bars: 'vertical' // Required for Material Bar Charts.
  }

  // xd: ChartBase

  objectsChartType: ChartType = ChartType.Bar

  @Input("ready")
  ready: boolean = false;

  columns: any = ["Country", "Precision", "Recall", "Fallout"]
  ratioChartType: ChartType = ChartType.PieChart;

  objectData: any[][]=[];

  constructor() {
  }

  ngOnInit(): void {
  }

  getRows(): any {
    this.objectData = []
   this.result.objectStatistics.forEach(
     (stats) => {
       let row = [stats.objectName, stats.precision, stats.recall, stats.fallout];
       this.objectData.push(row);
     }
   );
  }
}
