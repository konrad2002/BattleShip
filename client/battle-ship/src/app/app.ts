import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {Game} from './content/game/game';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Game],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  protected readonly title = signal('battle-ship');
}
