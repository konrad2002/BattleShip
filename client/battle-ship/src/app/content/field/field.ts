import {Component, Input} from '@angular/core';

@Component({
  selector: 'app-field',
  imports: [],
  templateUrl: './field.html',
  styleUrl: './field.scss'
})
export class Field {
  @Input() field!: number;
}
