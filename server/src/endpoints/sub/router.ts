import { createRouteMap, createRouter } from "https://deno.land/x/reno@v2.0.81/reno/router.ts";

import { create } from "./create.ts"

const routeMap = createRouteMap([
    ["create", create]
]);

export const submissionRouter = createRouter(routeMap);