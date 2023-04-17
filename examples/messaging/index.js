function _start() {
    let m = pub.open("my-messaging");
    pub.publish(m, "Hello World!", "slightjs");
}