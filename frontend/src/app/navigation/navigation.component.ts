import {AfterViewInit, Component} from '@angular/core';
import {Router} from "@angular/router";
import {NavigationEntry, NavigationService} from "../shared/navigation.service";
import {VisualizationService} from "../shared/visualization.service";

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.css']
})
export class NavigationComponent implements AfterViewInit {

  activeEntry: NavigationEntry = NavigationEntry.NONE;
  enum = NavigationEntry;

  constructor(private router: Router, private navigationService: NavigationService, private visualizationService: VisualizationService) {}

  ngAfterViewInit(): void {
    this.navigationService.navigationEntryChangedSubject.subscribe(
      value => {
        this.activeEntry = this.navigationService.currentlyActiveEntry;
      }
    );
  }
  navigateToPath(path: string, navigationEntry: NavigationEntry) {
    this.router.navigate([path]);
    this.navigationService.setActiveEntry(navigationEntry);
  }
  listResults() {
    this.navigateToPath("results", NavigationEntry.LIST_RESULTS);
  }
  multipleClassificationVisualizer() {
    this.navigateToPath("visualize-multiple", NavigationEntry.MULTIPLE_CLASSIFICATION_VISUALIZER);
  }
  executeClassification() {
    this.navigateToPath("execute", NavigationEntry.EXECUTE_CLASSIFICATION);
  }

  classificationVisualizer() {
    this.router.navigate(["visualize", this.visualizationService.singleClassificationDataId]);
    this.navigationService.setActiveEntry(NavigationEntry.CLASSIFICATION_VISUALIZER);
  }

  isThisActiveEntry(path: string) {


  }
}
