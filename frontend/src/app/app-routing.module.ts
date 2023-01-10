import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {ListResultsComponent} from "./statistics/list-results/list-results.component";
import {ExecuteClassificationComponent} from "./execution/execute-classification/execute-classification.component";
import {ResultVisualizationComponent} from "./statistics/result-visualization/result-visualization.component";
import {
  MultipleResultVisualizationComponent
} from "./statistics/multiple-result-visualization/multiple-result-visualization.component";

const routes: Routes = [
  {
    path: "results",
    component: ListResultsComponent
  },
  {
    path: "execute",
    component: ExecuteClassificationComponent
  },
  {
    path: "visualize",
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
