import { CoinsInOneErgo, ErgoDecimalPlacesNum } from "../common/constant.ts";
import { StatsService } from "../services/services.ts";
import { Context } from "./../types/core/context.ts";

export class InfoController {
  constructor(private statsService: StatsService) {}

  getInfo = async (ctx: Context) => {
    const info = await this.statsService.getBlockChainInfo();
    ctx.response.body = info;
  };

  getSupply = async (ctx: Context) => {
    const info = await this.statsService.getBlockChainInfo();
    const supply = (info.supply / CoinsInOneErgo).toFixed(ErgoDecimalPlacesNum);
    ctx.response.body = supply;
  };
}
