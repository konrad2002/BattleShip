import { TestBed } from '@angular/core/testing';

import { BattleShip } from './battle-ship';

describe('BattleShip', () => {
  let service: BattleShip;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(BattleShip);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
