import { InfoService } from "../services/InfoService.ts";
import { Router } from "./../deps.ts";
import { Context } from "./../types/core/context.ts";

const router: Router = new Router();
const infoService = new InfoService();

router.get("", (ctx: Context) => {
  ctx.response.body = "hello world";
});

// Info
router
  .get("/api/v0/info", infoService.getInfo)
  .get("/api/v0/info/supply", infoService.getSupply);

export { router };
