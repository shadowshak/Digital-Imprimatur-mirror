import { listenAndServe, ServerRequest } from "https://deno.land/std@v0.177.0/http/server.ts";

import {
  createRouter,
  AugmentedRequest,
  createRouteMap,
  textResponse,
  jsonResponse,
} from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";


export const routes = createRouteMap([
  ["/helloReno", () => textResponse("Response from Reno Server")],

  ["/swapi", async (req: AugmentedRequest) => {
    
    const res = await fetch(
      `https://swapi.dev/api/people`,
    );

    return jsonResponse(await res.json());
  }],

  
]);


const router = createRouter(routes);

(async () => {
  console.log("Listening for requests...");

  await listenAndServe(
    ":8001",
    async (req: ServerRequest) => {
      try {
        const res = await router(req);
        return req.respond(res);
      } catch (e) {
        return req.respond(e);
      }
    },
  );
})();