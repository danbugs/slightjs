function _start() {
    let s = sql.open("my-db");
    sql.exec(s, statement.prepare("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)", []));

    let name = random_name();
    console.log("Inserting user: " + name);

    sql.exec(s, statement.prepare("INSERT INTO users (name) VALUES (?)", [name]));

    let all_users = sql.query(s, statement.prepare("SELECT name FROM users", []));
    console.log(JSON.stringify(all_users));

    let one_user = sql.query(s, statement.prepare("SELECT name FROM users WHERE id = ?", [2]));
    console.log(JSON.stringify(one_user));
}

// create a random name
function random_name() {
    let name = "";
    for (let i = 0; i < 10; i++) {
        name += String.fromCharCode(Math.floor(Math.random() * 26) + 97);
    }
    return name;
}