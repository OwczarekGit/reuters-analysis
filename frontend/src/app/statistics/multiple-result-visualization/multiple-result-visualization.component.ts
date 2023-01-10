import {Component, OnInit} from '@angular/core';
import {NavigationEntry, NavigationService} from "../../shared/navigation.service";

@Component({
  selector: 'app-multiple-result-visualization',
  templateUrl: './multiple-result-visualization.component.html',
  styleUrls: ['./multiple-result-visualization.component.css']
})
export class MultipleResultVisualizationComponent implements OnInit {


  constructor(private navigationService: NavigationService) {
    this.navigationService.setActiveEntry(NavigationEntry.MULTIPLE_CLASSIFICATION_VISUALIZER);
  }

  ngOnInit(): void {
  }

}
