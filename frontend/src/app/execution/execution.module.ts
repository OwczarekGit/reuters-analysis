import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ExecuteClassificationComponent } from './execute-classification/execute-classification.component';
import {ReactiveFormsModule} from "@angular/forms";
import {HttpClientModule} from "@angular/common/http";



@NgModule({
    declarations: [
        ExecuteClassificationComponent
    ],
    exports: [
        ExecuteClassificationComponent
    ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
     HttpClientModule
  ]
})
export class ExecutionModule { }
