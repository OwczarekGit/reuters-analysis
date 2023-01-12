import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormControl, FormGroup, Validators} from "@angular/forms";
import {HttpClient} from "@angular/common/http";
import {ClassificationParameters} from "../classification-parameters";
import {Result} from "../../shared/result";
import {VisualizationService} from "../../shared/visualization.service";
import {ResultService} from "../../shared/result.service";
import {Router} from "@angular/router";
import {NavigationEntry, NavigationService} from "../../shared/navigation.service";

@Component({
  selector: 'app-execute-classification',
  templateUrl: './execute-classification.component.html',
  styleUrls: ['./execute-classification.component.css']
})
export class ExecuteClassificationComponent implements OnInit {

  waitingForResponse: boolean = false;

  formGroup: FormGroup;
  k: FormControl = new FormControl<number>(3, [Validators.required]);
  algorithm: FormControl = new FormControl<string>('chebyshev', [Validators.required]);
  ratio: FormControl = new FormControl<number>(50, [Validators.required]);
  visualizeResult: FormControl = new FormControl<boolean>(false, []);
  showResult: FormControl = new FormControl<boolean>(false, []);


  constructor(
    fb: FormBuilder,
    private http: HttpClient,
    private visualizationService: VisualizationService,
    private resultService: ResultService,
    private navigationService: NavigationService,
    private router: Router
  ) {
    this.navigationService.setActiveEntry(NavigationEntry.EXECUTE_CLASSIFICATION);
    this.formGroup = fb.group({
      k: this.k,
      algorithm: this.algorithm,
      ratio: this.ratio,
      visualizeResult: this.visualizeResult,
      showResult: this.showResult
    });
    let sound = document.createElement('audio')
    sound.src = 'assets/thunder.mp3'
    sound.volume = 0.2
    sound.play()
  }

  ngOnInit(): void {
  }

  executeClassification() {
    this.waitingForResponse = true;
    if(this.visualizeResult.value) {
      this.visualizationService.markSingleVisualizationAsUnready();
    }
    let classificationParams: ClassificationParameters = {
      k: this.k.value,
      algorithm: this.algorithm.value.toUpperCase(),
      ratio: this.ratio.value / 100
    };
    this.http.post<Result>("api/execute", classificationParams).subscribe(
      response => {
        this.waitingForResponse = false;
        if(this.visualizeResult.value) {
          this.visualizationService.setSingleClassificationDataId(response.id);
          if(this.showResult.value) {
            this.router.navigate(["visualize", response.id]);
            this.navigationService.setActiveEntry(NavigationEntry.CLASSIFICATION_VISUALIZER);
          }
        }
        this.resultService.addResult(response);
      }
    );
  }
}
