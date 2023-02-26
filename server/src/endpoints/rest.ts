import {
    createRouter,
    createRouteMap,
  } from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

  import { serveListener } from "https://deno.land/std@v0.177.0/http/server.ts";

import { submissionRouter } from "./sub/router.ts";

///
/// Creates a new response with the given status and message.
///
function createErrorResponse(status: number, { message }: Error) {
    return new Response(message, { status });
}

function map_error_response(e: Error) {
    return createErrorResponse(500, e);
}

export async function start_rest_server(port = 8080) {
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
                return map_error_response(e);
            }
        });
}