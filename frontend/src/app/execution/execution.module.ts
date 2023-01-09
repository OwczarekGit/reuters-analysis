import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ExecuteClassificationComponent } from './execute-classification/execute-classification.component';
import {ReactiveFormsModule} from "@angular/forms";
import {HttpClientModule} from "@angular/common/http";
import {SharedModule} from "../shared/shared.module";



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
    HttpClientModule,
    SharedModule
  ]
})
export class ExecutionModule { }
