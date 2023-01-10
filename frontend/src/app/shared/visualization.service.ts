import { Injectable } from '@angular/core';
import {Result} from "./result";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class VisualizationService {

  singleClassificationVisualizationReady: boolean;
  multipleClassificationVisualizationReady: boolean;

  singleClassificationData!: Result;
  multipleClassificationData!: Result[];

  singleClassificationDataReady: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);
  multipleClassificationDataReady: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  constructor() {
    this.singleClassificationVisualizationReady = false;
    this.multipleClassificationVisualizationReady = false;
  }

  setSingleClassificationData(data: Result) {
    this.singleClassificationData = data;
    this.singleClassificationDataReady.next(true);
  }
  setMultipleClassificationData(data: Result[]) {
    this.multipleClassificationData = data;
    this.multipleClassificationDataReady.next(true);
  }

  markSingleVisualizationAsUnready() {
    this.singleClassificationVisualizationReady = false;
  }

  markSingleVisualizationAsReady() {
    this.singleClassificationVisualizationReady = true;
  }
  markMultipleVisualizationAsUnready() {
    this.multipleClassificationVisualizationReady = false;
  }

  markMultipleVisualizationAsReady() {
    this.multipleClassificationVisualizationReady = true;
  }
}
