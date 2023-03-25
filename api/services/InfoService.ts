import { Context } from "./../types/core/context.ts";

export class InfoService {
  constructor() {
  }

  getInfo = (ctx: Context) => {
    ctx.response.body = "hello world";
  };

  getSupply = (ctx: Context) => {
    ctx.response.body = "hello world";
  };
}
