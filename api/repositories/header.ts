import { DbPool } from "./db/db.ts";

export class HeaderRepo {
  constructor(private dbPool: DbPool) {}
}
