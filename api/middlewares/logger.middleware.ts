import { Context } from "./../types/core/context.ts";
import { log } from "./../deps.ts";

const loggerMiddleware = async (ctx: Context, next: () => Promise<unknown>) => {
  await next();
  const reqTime = ctx.response.headers.get("X-Response-Time");
  const reqId = ctx.response.headers.get("X-Response-Id");
  const status = ctx.response.status;
  log.info(
    `${reqId} ${ctx.request.method} ${ctx.request.url} - ${reqTime} status: ${status}`,
  );
};

export { loggerMiddleware };
