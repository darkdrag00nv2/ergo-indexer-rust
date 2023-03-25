import { Context } from "./../types/core/context.ts";

const requestIdMiddleware = async (ctx: Context, next: () => Promise<void>) => {
  let requestId = ctx.request.headers.get("X-Response-Id");
  if (!requestId) {
    requestId = crypto.randomUUID();
    ctx.request.headers.set("X-Response-Id", requestId);
  }
  await next();
  ctx.response.headers.set("X-Response-Id", requestId);
};

export { requestIdMiddleware };
