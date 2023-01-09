import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {FormBuilder, FormControl, FormGroup, Validators} from "@angular/forms";
import {HttpClient} from "@angular/common/http";
import {ClassificationParameters} from "../classification-parameters";
import {Result} from "../../shared/result";

@Component({
  selector: 'app-execute-classification',
  templateUrl: './execute-classification.component.html',
  styleUrls: ['./execute-classification.component.css']
})
export class ExecuteClassificationComponent implements OnInit {

  @Output("classificationResult")
  classificationResultEmitter: EventEmitter<Result> = new EventEmitter<Result>();

  formGroup: FormGroup;
  k: FormControl = new FormControl<number>(3, [Validators.required]);
  algorithm: FormControl = new FormControl<string>('chebyshev', [Validators.required]);
  ratio: FormControl = new FormControl<number>(50, [Validators.required]);


  constructor(
    fb: FormBuilder,
    private http: HttpClient
  ) {
    this.formGroup = fb.group({
      k: this.k,
      algorithm: this.algorithm,
      ratio: this.ratio
    });
  }

  ngOnInit(): void {
  }

  executeClassification() {
    let classificationParams: ClassificationParameters = {
      k: this.k.value,
      algorithm: this.algorithm.value.toUpperCase(),
      ratio: this.ratio.value / 100
    };
    this.http.post<Result>("api/execute", classificationParams).subscribe(
      value => this.classificationResultEmitter.emit(value)
    );
  }
}
