// #! wit/keyvalue.wit

function _start() {
    let kv = keyvalue.open("my-container");
    console.log(kv);
}