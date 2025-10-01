import {Component, OnInit} from '@angular/core';
import {BattleShipService} from '../../core/services/api/battle-ship';
import {Board} from '../../core/model/board.model';
import {Field} from '../field/field';

@Component({
  selector: 'app-game',
  imports: [
    Field
  ],
  templateUrl: './game.html',
  styleUrl: './game.scss'
})
export class Game implements OnInit {

  board?: Board;

  constructor(
    private battleShipService: BattleShipService
  ) {}

  ngOnInit() {
    this.battleShipService.getBoard(1).subscribe(board => {
      this.board = board;
    })
  }

  shoot(number: number, $index: number) {
    this.battleShipService.shoot(number, $index).subscribe(shoots => {
      this.board = shoots;
    })
  }
}
