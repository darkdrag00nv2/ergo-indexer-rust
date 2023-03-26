import { DbPool } from "./db/db.ts";

export class TransactionRepo {
  constructor(private dbPool: DbPool) {}

  /// Get total number of transactions appeared in the main chain after a given timestamp ts.
  async countMainSince(_ts: number): Promise<number> {
    return 0;
  }
}
