import { Component, OnInit } from '@angular/core';
import {ResultService} from "../../../shared/result.service";
import {Result} from "../../../shared/result";

@Component({
  selector: 'app-import-results',
  templateUrl: './import-results.component.html',
  styleUrls: ['./import-results.component.css']
})
export class ImportResultsComponent implements OnInit {

  file: File | undefined
  constructor(private resultService: ResultService) { }

  ngOnInit(): void {
  }

  onChosenFileChange($event: any) {
    this.file = $event.target.files[0];
  }

  importResults() {
    if(this.file != undefined) {
      let fileReader: FileReader = new FileReader();
      fileReader.onload = () => {
        let fileContent = fileReader.result as string;
        let results: Result[] = JSON.parse(fileContent);
        this.resultService.addNewResults(results);
      }
      fileReader.readAsText(this.file);
    }
  }
}
