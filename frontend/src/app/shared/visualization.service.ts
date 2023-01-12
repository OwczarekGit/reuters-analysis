import { Injectable } from '@angular/core';
import {Result} from "./result";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class VisualizationService {

  singleClassificationVisualizationReady: boolean;
  multipleClassificationVisualizationReady: boolean;

  singleClassificationDataId: number | undefined;
  multipleClassificationData!: Result[];

  singleClassificationDataReady: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);
  multipleClassificationDataReady: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  constructor() {
    this.singleClassificationVisualizationReady = false;
    this.multipleClassificationVisualizationReady = false;
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

  setSingleClassificationDataId(id: number | undefined) {
    this.singleClassificationDataId =  id;
  }
}
