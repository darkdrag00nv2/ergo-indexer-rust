import { InfoController } from "../controllers/controllers.ts";
import { BlockInfoRepo, HeaderRepo } from "../repositories/repositories.ts";
import { StatsService } from "../services/services.ts";
import { Router } from "./../deps.ts";
import { Context } from "./../types/core/context.ts";

const router: Router = new Router();
const headerRepo = new HeaderRepo();
const blockInfoRepo = new BlockInfoRepo();
const statsService = new StatsService(headerRepo, blockInfoRepo);
const infoController = new InfoController(statsService);

router.get("", (ctx: Context) => {
  ctx.response.body = "hello world";
});

// Info
router
  .get("/api/v0/info", infoController.getInfo)
  .get("/api/v0/info/supply", infoController.getSupply);

export { router };
