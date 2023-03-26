import { config } from "../../common/config.ts";
import { postgres } from "./../../deps.ts";

export class DbPool {
  private pool: postgres.Pool;

  constructor() {
    const dbOptions: postgres.ClientOptions = {
      database: config.DATABASE_NAME,
      hostname: config.DATABASE_HOST,
      user: config.DATABASE_USER,
      password: config.DATABASE_PWD,
      port: config.DATABASE_PORT,
    };

    this.pool = new postgres.Pool(
      dbOptions,
      Number(config.DB_NUM_CONNECTIONS),
      /*lazy=*/ true,
    );
  }

  async get_connection(): Promise<postgres.PoolClient> {
    return await this.pool.connect();
  }
}
