import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ChartIconComponent } from './chart-icon/chart-icon.component';
import { AccuracyIconComponent } from './accuracy-icon/accuracy-icon.component';
import { EmptyIconComponent } from './empty-icon/empty-icon.component';
import { KIconComponent } from './k-icon/k-icon.component';
import { RefreshIconComponent } from './refresh-icon/refresh-icon.component';
import { ListIconComponent } from './list-icon/list-icon.component';
import { LightningIconComponent } from './lightning-icon/lightning-icon.component';
import { Chart2IconComponent } from './chart2-icon/chart2-icon.component';
import { TrashIconComponent } from './trash-icon/trash-icon.component';



@NgModule({
    declarations: [
        ChartIconComponent,
        AccuracyIconComponent,
        EmptyIconComponent,
        KIconComponent,
        RefreshIconComponent,
        ListIconComponent,
        LightningIconComponent,
        Chart2IconComponent,
        TrashIconComponent
    ],
  exports: [
    ChartIconComponent,
    AccuracyIconComponent,
    EmptyIconComponent,
    KIconComponent,
    RefreshIconComponent,
    ListIconComponent,
    LightningIconComponent,
    Chart2IconComponent,
    TrashIconComponent
  ],
    imports: [
        CommonModule
    ]
})
export class SharedModule { }
