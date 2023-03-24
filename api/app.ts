import { Application } from "./deps.ts";
import router from "./routes.ts";
import "https://deno.land/x/dotenv@v3.2.2/load.ts";
import * as log from "https://deno.land/std/log/mod.ts";

const env = Deno.env.toObject();
const PORT = env.PORT || 3000;
const HOST = env.HOST || "localhost";

const app = new Application();

// Logger
app.use(async (ctx, next) => {
    await next();
    const rt = ctx.response.headers.get("X-Response-Time");
    log.info(`${ctx.request.method} ${ctx.request.url} - ${rt}`);
});

// Timing
app.use(async (ctx, next) => {
    const start = Date.now();
    await next();
    const ms = Date.now() - start;
    ctx.response.headers.set("X-Response-Time", `${ms}ms`);
});

app.use(router.routes());
app.use(router.allowedMethods());

log.info(`Server running on port ${PORT}`);

app.listen(`${HOST}:${PORT}`);
