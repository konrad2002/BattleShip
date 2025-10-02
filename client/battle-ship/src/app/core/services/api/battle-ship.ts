import { Injectable } from '@angular/core';
import {BaseService} from './base.service';
import {ApiService} from './api.service';
import {Observable} from 'rxjs';
import {Board} from '../../model/board.model';

@Injectable({
  providedIn: 'root'
})
export class BattleShipService extends BaseService {
  constructor(private apiService: ApiService) {
    super("BattleShipService", "http://localhost:8300/");
  }

  getBoard(side: number): Observable<Board> {
    return this.apiService.get(this.API_URL, `board/${side}`);
  }

  init(side: number): Observable<Board> {
    let initState = [
      0,0,2,2,0,
      0,0,0,0,0,
      2,0,2,0,0,
      2,0,0,2,2,
      2,0,2,0,0,
    ]
    return this.apiService.post(this.API_URL, `board/${side}/init`, initState);
  }

  shoot(side: number, field: number): Observable<Board> {
    return this.apiService.post(this.API_URL, `board/${side}/shoot/${field}`, null);
  }
}
