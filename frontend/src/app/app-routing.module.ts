import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {ListResultsComponent} from "./statistics/list-results/list-results.component";
import {ExecuteClassificationComponent} from "./execution/execute-classification/execute-classification.component";
import {ResultVisualizationComponent} from "./statistics/result-visualization/result-visualization.component";
import {
  MultipleResultVisualizationComponent
} from "./statistics/multiple-result-visualization/multiple-result-visualization.component";
import {ResultsSettingsComponent} from "./statistics/results-settings/results-settings.component";

const routes: Routes = [
  {
    path: "results",
    component: ListResultsComponent
  },
  {
    path: "results-settings",
    component: ResultsSettingsComponent
  },
  {
    path: "execute",
    component: ExecuteClassificationComponent
  },
  {
    path: "visualize/:id",
    component: ResultVisualizationComponent
  },
  {
    path: "visualize-multiple",
    component: MultipleResultVisualizationComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
