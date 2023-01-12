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
import { FlaskIconComponent } from './flask-icon/flask-icon.component';
import { ArrowIconComponent } from './arrow-icon/arrow-icon.component';
import { FloppyIconComponent } from './floppy-icon/floppy-icon.component';
import { SettingsIconComponent } from './settings-icon/settings-icon.component';
import { FolderIconComponent } from './folder-icon/folder-icon.component';
import { UploadIconComponent } from './upload-icon/upload-icon.component';



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
        TrashIconComponent,
        FlaskIconComponent,
        ArrowIconComponent,
        FloppyIconComponent,
        SettingsIconComponent,
        FolderIconComponent,
        UploadIconComponent
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
    TrashIconComponent,
    FlaskIconComponent,
    ArrowIconComponent,
    FloppyIconComponent,
    SettingsIconComponent,
    FolderIconComponent,
    UploadIconComponent
  ],
    imports: [
        CommonModule
    ]
})
export class SharedModule { }
