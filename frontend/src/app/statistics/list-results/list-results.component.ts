import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Result} from "../../shared/result";
import {HttpClient} from "@angular/common/http";
import {ResultService} from "../../shared/result.service";
import {VisualizationService} from "../../shared/visualization.service";
import {Router} from "@angular/router";
import {NavigationEntry, NavigationService} from "../../shared/navigation.service";

@Component({
  selector: 'app-list-results',
  templateUrl: './list-results.component.html',
  styleUrls: ['./list-results.component.css']
})
export class ListResultsComponent implements OnInit {
  results: Result[] = [];

  @Output("visualizeResult")
  visualizeResultEmitter: EventEmitter<Result> = new EventEmitter<Result>();

  constructor(
    private http: HttpClient,
    private resultService: ResultService,
    private visualizationService: VisualizationService,
    private navigationService: NavigationService,
    private router: Router
  ) {
    this.navigationService.setActiveEntry(NavigationEntry.LIST_RESULTS);
    this.resultService.resultsSubject.subscribe(
      results => this.results = results
    );
  }
  activeIndex: number = -1;

  ngOnInit(): void {
  }

  visualizeResult(index: number) {
    this.visualizationService.markSingleVisualizationAsUnready();
    this.router.navigate(["visualize"]);
    this.navigationService.setActiveEntry(NavigationEntry.CLASSIFICATION_VISUALIZER);
    this.visualizationService.setSingleClassificationData(this.results[index]);
  }

  deleteResult(index: number) {
   this.resultService.deleteResult(index);
   this.activeIndex = -1;
  }

  refresh() {
    this.resultService.loadData();
  }

  setActive(index: number) {
    if(index === this.activeIndex) {
      this.activeIndex = -1;
    } else {
      this.activeIndex = index;
    }

  }
}
