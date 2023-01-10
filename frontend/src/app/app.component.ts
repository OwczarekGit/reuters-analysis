import {AfterViewInit, Component, ElementRef, ViewChild} from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements AfterViewInit{

  @ViewChild('nav_box')
  navBox!: ElementRef<HTMLElement>

  @ViewChild('content_box')
  contentBox!: ElementRef<HTMLDivElement>

  ngAfterViewInit(){
    this.navBox.nativeElement.addEventListener('mouseenter', (ev) =>{
      this.contentBox.nativeElement.style.width = 'calc( 100% - 160px )';
      this.navBox.nativeElement.style.width = '160px';
    })

    this.navBox.nativeElement.addEventListener('mouseleave', (ev) => {
      this.contentBox.nativeElement.style.width = 'calc( 100% - 54px )';
      this.navBox.nativeElement.style.width = '54px';
    })

  }

}
