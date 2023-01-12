import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {ClassificationObjectStatistics, Result} from "../../../shared/result";
import {HttpClient} from "@angular/common/http";
import {ResultService} from "../../../shared/result.service";
import {VisualizationService} from "../../../shared/visualization.service";
import {NavigationEntry, NavigationService} from "../../../shared/navigation.service";
import {Router} from "@angular/router";
import {DomSanitizer, SafeUrl} from "@angular/platform-browser";

@Component({
  selector: 'app-export-results',
  templateUrl: './export-results.component.html',
  styleUrls: ['./export-results.component.css']
})
export class ExportResultsComponent {

  selectedResults: Result[] = [];
  results: Result[] = [];

  @Output("visualizeResult")
  visualizeResultEmitter: EventEmitter<Result> = new EventEmitter<Result>();

  constructor(
    private http: HttpClient,
    private resultService: ResultService,
    private visualizationService: VisualizationService,
    private navigationService: NavigationService,
    private router: Router,
    private sanitizer: DomSanitizer
  ) {
    this.navigationService.setActiveEntry(NavigationEntry.LIST_RESULTS);
    this.resultService.resultsSubject.subscribe(
      results => this.results = results
    );
  }

  exportSelectedResultsAsJson() {
    let resultsWithoutId: Result[] = [];
    for(let res of this.selectedResults) {
      let objStats: ClassificationObjectStatistics[] = [];
      for(let objStat of res.objectStatistics) {
        objStats.push(
          {
            objectName: objStat.objectName,
            precision: objStat.precision,
            recall: objStat.recall,
            fallout: objStat.fallout
          }
        );
      }
      resultsWithoutId.push(
        {
          creationDate: res.creationDate,
          algorithm: res.algorithm,
          k: res.k,
          splitRatio: res.splitRatio,
          trainingSliceSize: res.trainingSliceSize,
          testingSliceSize: res.testingSliceSize,
          accuracy: res.accuracy,
          objectStatistics: objStats
        }
      );
    }
    let sJson = JSON.stringify(resultsWithoutId);
    let element = document.createElement('a');
    element.setAttribute('href', "data:text/json;charset=UTF-8," + encodeURIComponent(sJson));
    element.setAttribute('download', "exportedResults.json");
    element.style.display = 'none';
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
  }

  setActive(i: number) {
    let result: Result = this.results[i];
    if(this.selectedResults.includes(result)) {
      let selectedIndex: number = this.selectedResults.indexOf(result);
      this.selectedResults.splice(selectedIndex, 1);
    } else {
      this.selectedResults.push(result);
    }
  }

  isActive(i: number) {
    return this.selectedResults.includes(this.results[i]);
  }

  exportAll() {
    this.selectedResults = this.results;
    this.exportSelectedResultsAsJson();
    this.selectedResults = [];
  }
}
