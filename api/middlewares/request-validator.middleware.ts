import { flattenMessages, validate, ValidationRules, httpErrors } from "../deps.ts";
import { Context } from "../types/core/context.ts";

/**
 * request validation middleware
 * validate request body with given validation rules
 */
const requestValidator = ({ bodyRules }: { bodyRules: ValidationRules }) => {
  return async (ctx: Context, next: () => Promise<unknown>) => {
    /** get request body */
    const request = ctx.request;
    const body = (await request.body()).value;

    if (!body) {
      throw new httpErrors.BadRequest("Body of the request is undefined");
    }

    /** check rules */
    const [isValid, errors] = await validate(body!, bodyRules);
    const flattenErrors = flattenMessages(errors);
    if (!isValid) {
      throw new httpErrors.BadRequest("TODO: Add message");
    }

    await next();
  };
};

export { requestValidator };
