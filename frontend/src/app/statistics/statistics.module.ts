import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ResultVisualizationComponent } from './result-visualization/result-visualization.component';
import {SharedModule} from "../shared/shared.module";
import {GoogleChartsModule} from "angular-google-charts";
import { ListResultsComponent } from './list-results/list-results.component';
import { MultipleResultVisualizationComponent } from './multiple-result-visualization/multiple-result-visualization.component';
import { ResultListEntryComponent } from './list-results/result-list-entry/result-list-entry.component';
import { ResultsSettingsComponent } from './results-settings/results-settings.component';
import { ImportResultsComponent } from './results-settings/import-results/import-results.component';
import { ExportResultsComponent } from './results-settings/export-results/export-results.component';
import { ExportResultEntryComponent } from './results-settings/export-results/export-result-entry/export-result-entry.component';



@NgModule({
  declarations: [
    ResultVisualizationComponent,
    ListResultsComponent,
    MultipleResultVisualizationComponent,
    ResultListEntryComponent,
    ResultsSettingsComponent,
    ImportResultsComponent,
    ExportResultsComponent,
    ExportResultEntryComponent
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
