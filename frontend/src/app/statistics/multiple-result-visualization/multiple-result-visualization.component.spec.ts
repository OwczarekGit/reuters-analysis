import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MultipleResultVisualizationComponent } from './multiple-result-visualization.component';

describe('MultipleResultVisualizationComponent', () => {
  let component: MultipleResultVisualizationComponent;
  let fixture: ComponentFixture<MultipleResultVisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ MultipleResultVisualizationComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MultipleResultVisualizationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
