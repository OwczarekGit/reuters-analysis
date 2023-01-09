import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Result} from "../../shared/result";
import {HttpClient} from "@angular/common/http";

@Component({
  selector: 'app-list-results',
  templateUrl: './list-results.component.html',
  styleUrls: ['./list-results.component.css']
})
export class ListResultsComponent implements OnInit {
  results: Result[] = [];

  @Output("visualizeResult")
  visializeResultEmitter: EventEmitter<Result> = new EventEmitter<Result>();

  constructor(private http: HttpClient) {
    this.http.get<Result[]>("api/result").subscribe(
      results => this.results = results
    );
  }

  ngOnInit(): void {
  }

  visualizeResult(index: number) {
    this.visializeResultEmitter.emit(this.results[index]);
  }

  deleteResult(index: number) {
    this.http.delete(`api/result/${this.results[index].id}`).subscribe(
      () => this.results.splice(index, 1)
    );
  }

  refresh() {
    this.http.get<Result[]>("api/result").subscribe(
      results => this.results = results
    );
  }
}
