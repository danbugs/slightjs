function _start() {
    let r = router.new();
    let r_with_route = router
        .get(r, "/hello", "handle_hello");
    console.log("Server listening on port 3000...");
    let s = server.serve("0.0.0.0:3000", r_with_route);    
}

function handle_hello(req) {
    console.log("I just got a request uri: ", req.uri, ", method: ", req.method);
    return JSON.stringify({ headers: null, body: "Hello, JS Wasm!", status: 200 });
}