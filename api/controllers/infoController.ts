import { StatsService } from "../services/services.ts";
import { Context } from "./../types/core/context.ts";

export class InfoController {
  constructor(private statsService: StatsService) {}

  getInfo = async (ctx: Context) => {
    const info = await this.statsService.getBlockChainInfo();
    ctx.response.body = info;
  };

  getSupply = (ctx: Context) => {
    ctx.response.body = "hello world";
  };
}
