function _start() {
    let rnd_num = random_number();
    let rnd_key = "THIS_IS_ANOTHER_TEST_CONFIG_" + rnd_num;

    let c = configs.open("my-secret-store");
    configs.set(c, rnd_key, "Hello, world!");
    console.log(fromUtf8(configs.get(c, rnd_key)));
}

function random_number() {
    return Math.floor(Math.random() * 1000000);
}