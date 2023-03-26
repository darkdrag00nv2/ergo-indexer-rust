import { config } from "../../config/config.ts";
import { postgres } from "./../../deps.ts";

export class DbPool {
  private pool: postgres.Pool;

  constructor() {
    const dbOptions: postgres.ClientOptions = {
      database: config.DB_NAME,
      hostname: config.DB_HOST,
      user: config.DB_USER,
      password: config.DB_PASS,
    };

    this.pool = new postgres.Pool(
      dbOptions,
      Number(config.DB_NUM_CONNECTIONS),
      true,
    );
  }

  get_connection(): Promise<postgres.PoolClient> {
    return this.pool.connect();
  }
}
