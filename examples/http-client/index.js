function _start() {
    console.log(http_client.request({ method: "GET", uri: "https://some-random-api.ml/facts/dog", headers: [], body: null, params: [] }));
}