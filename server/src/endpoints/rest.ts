import {
    createRouter,
    createRouteMap,
  } from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

  import { serveListener } from "https://deno.land/std@v0.177.0/http/server.ts";

import { submissionRouter } from "./sub/router.ts";
import { HttpError } from "../errors.ts";

///
/// Creates a new response with the given status and message.
///
function CreateErrorResponse(status: number, { message }: Error) {
    return new Response(message, { status });
}

function MapErrorResponse(e: Error) {
    let errorCode = 500;

    if (e instanceof HttpError) {
        errorCode = e.code;
    }

    return CreateErrorResponse(errorCode, e);
}

export async function StartRestServer(port = 8080) {
    const routes = createRouteMap([
        ["/sub", submissionRouter]
    ]);

    const router = createRouter(routes);

    const listener = Deno.listen({ port })

    await serveListener(
        listener,
        async (req) => {
            // log request

            try {
                const response = await router(req);
                return response;
            } catch (e) {
                return MapErrorResponse(e);
            }
        });
}