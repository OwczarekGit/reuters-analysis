import {AfterViewInit, Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {Result} from "../../shared/result";
import {ChartType} from "angular-google-charts";
import {VisualizationService} from "../../shared/visualization.service";
import {NavigationEntry, NavigationService} from "../../shared/navigation.service";
import {ResultService} from "../../shared/result.service";
import {ActivatedRoute} from "@angular/router";

@Component({
  selector: 'app-result-visualization',
  templateUrl: './result-visualization.component.html',
  styleUrls: ['./result-visualization.component.css']
})
export class ResultVisualizationComponent implements OnInit {

  result!: Result

  options:any = {
    chart: {
      title: 'Classification objects',
      subtitle: 'Precision, Recall, and Fallout stats',

    },
    bars: 'vertical'
  }


  objectsChartType: ChartType = ChartType.Bar

  columns: any = ["Country", "Precision", "Recall", "Fallout"]
  ratioChartType: ChartType = ChartType.PieChart;

  objectData: any[][]=[];
  ready: boolean = false;

  constructor(route: ActivatedRoute, public resultService: ResultService, private navigationService: NavigationService) {
    this.navigationService.setActiveEntry(NavigationEntry.CLASSIFICATION_VISUALIZER);
    let id: string | null = route.snapshot.paramMap.get("id");
    if(id != null) {
      let res = this.resultService.getResult(+id);
      if(res != undefined) {
        this.result = res;
        this.getRows();
        this.ready = true;
      }
    }
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
