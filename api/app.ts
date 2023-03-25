import { Application } from "./deps.ts";
import { router } from "./routes/routes.ts";
import { log, oakCors } from "./deps.ts";
import * as middlewares from "./middlewares/middlewares.ts";
import { config } from "./config/config.ts";

const PORT = config.PORT || 3000;
const HOST = config.HOST || "localhost";

const app = new Application();

app.use(oakCors());
app.use(middlewares.loggerMiddleware);
app.use(middlewares.errorMiddleware);
app.use(middlewares.timingMiddleware);

app.use(router.routes());
app.use(router.allowedMethods());

log.info(`Server running on port ${PORT}`);

app.listen(`${HOST}:${PORT}`);
