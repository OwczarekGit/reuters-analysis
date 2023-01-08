import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ResultVisualizationComponent } from './result-visualization.component';

describe('ResultVisualizationComponent', () => {
  let component: ResultVisualizationComponent;
  let fixture: ComponentFixture<ResultVisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ResultVisualizationComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ResultVisualizationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
