import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ResultVisualizationComponent } from './result-visualization/result-visualization.component';
import {SharedModule} from "../shared/shared.module";
import {GoogleChartsModule} from "angular-google-charts";
import { ListResultsComponent } from './list-results/list-results.component';



@NgModule({
  declarations: [
    ResultVisualizationComponent,
    ListResultsComponent
  ],
  exports: [
    ResultVisualizationComponent,
    ListResultsComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    GoogleChartsModule
  ]
})
export class StatisticsModule { }
