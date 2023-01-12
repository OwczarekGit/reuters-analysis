import { Component, OnInit } from '@angular/core';
import {Router} from "@angular/router";

@Component({
  selector: 'app-results-settings',
  templateUrl: './results-settings.component.html',
  styleUrls: ['./results-settings.component.css']
})
export class ResultsSettingsComponent implements OnInit {

  constructor(private router: Router) { }

  ngOnInit(): void {
  }

  backToResultList() {
    this.router.navigate(["results"]);
  }
}
