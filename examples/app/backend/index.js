function _start() {
    // start a HTTP router w/ the following routes:
    let r =
        router.post(
            router.get(router.new(),
                "/get/*", "handle_get"),
            "/post/*", "handle_post");

    // use messaging for logging/event-streaming
    pub.publish(pub.open("my-messaging"), "Server listening at port 3000...", "slightjs-event-streaming");

    // start the server
    let s = server.serve("0.0.0.0:3000", r);
}

// handler for GET requests
function handle_get(req) {
    pub.publish(pub.open("my-messaging"), "Received GET request.", "slightjs-event-streaming");

    // switch-statement for route control
    switch (req.uri) {
        case "/get/orders":
            pub.publish(pub.open("my-messaging"), "Hit route './get/order'.", "slightjs-event-streaming");

            let keys = keyvalue.keys(keyvalue.open("restaurant-data"));
            let key_string = "";
            for (let i = 0; i < keys.length; i++) {
                key_string += keys[i];
                if (i < keys.length - 1) {
                    key_string += ", ";
                }
            }

            // access the key-value store's keys, and return them as a JSON response
            return JSON.stringify({ headers: null, body: key_string, status: 200 });
        default:

            // return a 204 response
            return handle_unrecognized_route();
    }

}

// handler for POST requests
function handle_post(req) {
    pub.publish(pub.open("my-messaging"), "Received POST request.", "slightjs-event-streaming");
    switch (req.uri) {
        case "/post/order":
            pub.publish(pub.open("my-messaging"), "Hit route './post/order'.", "slightjs-event-streaming");

            // access the key-value store, and set the keys w/ orders
            keyvalue.set(keyvalue.open("restaurant-data"), req.body, "fake data");
            return JSON.stringify({ headers: null, body: null, status: 200 });
        default:
            return handle_unrecognized_route();
    }
}

function handle_unrecognized_route() {
    pub.publish(pub.open("my-messaging"), "Hit unrecognized route.", "slightjs-event-streaming");
    return JSON.stringify({ headers: null, body: null, status: 204 });
}