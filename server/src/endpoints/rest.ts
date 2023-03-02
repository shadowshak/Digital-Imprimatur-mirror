import { serveListener } from "https://deno.land/std@v0.177.0/http/server.ts";
import {
    createRouter,
    createRouteMap,
} from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

import { HttpError } from "../errors.ts";

import {
    sub_create,
    sub_read,
    sub_update,
    sub_delete,
    sub_read_feedback,
    sub_read_docs,
 } from './sub/mod.ts'

///
/// Creates a new response with the given status and message.
///
function CreateErrorResponse(status: number, { message }: Error) {
    return new Response(message, { status });
}

///
/// Given an error, returns a response with the appropriate status code.
///
/// If the error has an explicit error code, use that
/// Otherwise, use 500 (internal server error)
///
function MapErrorResponse(e: Error) {
    let errorCode = 500;

    if (e instanceof HttpError) {
        errorCode = e.code;
    }

    return CreateErrorResponse(errorCode, e);
}

///
/// Starts the REST server on the given port.
///
/// If no port is given, port 8080 is used.
///
export async function StartRestServer(port = 8080) {
    const routes = createRouteMap([
        ["/sub/create", sub_create],
        ["/sub/read", sub_read],
        ["/sub/update", sub_update],
        ["/sub/delete", sub_delete],
        ["/sub/read/docs", sub_read_docs],
        ["/sub/read/feedback", sub_read_feedback],
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