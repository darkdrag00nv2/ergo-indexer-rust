import { Application } from "./deps.ts";
import router from "./routes.ts";
import "https://deno.land/x/dotenv@v3.2.2/load.ts";

const env = Deno.env.toObject();
const PORT = env.PORT || 3000;
const HOST = env.HOST || "localhost";

const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

console.log(`Server running on port ${PORT}`);

app.listen(`${HOST}:${PORT}`);
