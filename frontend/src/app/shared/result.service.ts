import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Result} from "./result";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class ResultService {
  results: Result[] = [];
  resultsSubject: BehaviorSubject<Result[]> = new BehaviorSubject<Result[]>(this.results);
  constructor(private http: HttpClient) {
    this.loadData();
  }
  loadData() {
    this.http.get<Result[]>("api/result").subscribe(
      response => {
        this.results = response;
        this.resultsSubject.next(response);
      }
    );
  }
  addResult(result: Result) {
    this.results.push(result);
    this.resultsSubject.next(this.results);
  }
  deleteResult(index: number) {
    this.http.delete(`api/result/${this.results[index].id}`).subscribe(
      () => {
        this.results.splice(index, 1);
        this.resultsSubject.next(this.results);
      }
    );
  }

  addNewResults(results: Result[]) {
    this.http.post<Result[]>("api/result/import", results).subscribe(
      (response) => {
        this.results.push(...response);
        this.resultsSubject.next(this.results);
      }
    )
  }
}
