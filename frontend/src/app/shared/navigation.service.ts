import { Injectable } from '@angular/core';
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class NavigationService {

  currentlyActiveEntry: NavigationEntry = NavigationEntry.NONE;
  navigationEntryChangedSubject: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  constructor() { }

  setActiveEntry(navigationEntry: NavigationEntry) {
    this.currentlyActiveEntry = navigationEntry;
    this.navigationEntryChangedSubject.next(true);
  }
}
export enum NavigationEntry {
  NONE,
  LIST_RESULTS,
  EXECUTE_CLASSIFICATION,
  MULTIPLE_CLASSIFICATION_VISUALIZER,
  CLASSIFICATION_VISUALIZER
}
