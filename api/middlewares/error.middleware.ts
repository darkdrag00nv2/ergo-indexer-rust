import { isHttpError, Status } from "./../deps.ts";
import { config } from "./../common/config.ts";
import { Context } from "./../types/core/context.ts";

const errorMiddleware = async (ctx: Context, next: () => Promise<unknown>) => {
  try {
    await next();
  } catch (err) {
    let message = err.message;
    const status = err.status || err.statusCode || Status.InternalServerError;

    if (!isHttpError(err)) {
      message = config.ENV === "dev" || config.ENV === "development"
        ? message
        : "Internal Server Error";
    }

    if (config.ENV === "dev" || config.ENV === "development") {
      console.log(err);
    }

    ctx.response.status = status;
    ctx.response.body = { status, message };
  }
};

export { errorMiddleware };
