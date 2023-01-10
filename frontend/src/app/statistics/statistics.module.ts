import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ResultVisualizationComponent } from './result-visualization/result-visualization.component';
import {SharedModule} from "../shared/shared.module";
import {GoogleChartsModule} from "angular-google-charts";
import { ListResultsComponent } from './list-results/list-results.component';
import { MultipleResultVisualizationComponent } from './multiple-result-visualization/multiple-result-visualization.component';
import { ResultListEntryComponent } from './list-results/result-list-entry/result-list-entry.component';



@NgModule({
  declarations: [
    ResultVisualizationComponent,
    ListResultsComponent,
    MultipleResultVisualizationComponent,
    ResultListEntryComponent
  ],
  exports: [
    ResultVisualizationComponent,
    ListResultsComponent,
    MultipleResultVisualizationComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    GoogleChartsModule
  ]
})
export class StatisticsModule { }
