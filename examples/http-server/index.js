function _start() {
    let r = router.delete(
                router.put(
                    router.post(
                        router.get(router.new(), 
                        "/get/*", "handle_get"),
                    "/post/*", "handle_post"), 
                "/put/*", "handle_put"), 
            "/delete/*", "handle_delete");

    console.log("Server listening on port 3000...");
    let s = server.serve("0.0.0.0:3000", r);
}

function handle_get(req) {
    console.log("I just got a request @ uri: ", req.uri, ", w/ method: ", req.method);
    switch (req.uri) {
        case "/get/hello":
            console.log("hello!");
            break;
        case "get/world":
            console.log("world!");
            break;
        default:
            break;
    }

    return JSON.stringify({ headers: null, body: "Hello, JS Wasm!", status: 200 });
}

function handle_post(req) {
    console.log("I just got a request @ uri: ", req.uri, ", w/ method: ", req.method);
    return JSON.stringify({ headers: null, body: "Hello, JS Wasm!", status: 200 });
}

function handle_put(req) {
    console.log("I just got a request @ uri: ", req.uri, ", w/ method: ", req.method);
    return JSON.stringify({ headers: null, body: "Hello, JS Wasm!", status: 200 });
}

function handle_delete(req) {
    console.log("I just got a request @ uri: ", req.uri, ", w/ method: ", req.method);
    return JSON.stringify({ headers: null, body: "Hello, JS Wasm!", status: 200 });
}
