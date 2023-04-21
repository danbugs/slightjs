function _start() {
    let bucket = container.open("slight-example-bucket");
    if (container.has_object(bucket, "file.txt")) {
        let rs = container.read_object(bucket, "file.txt");
        let contents = read_stream.read(rs, 1024);
        console.log("The contents of file.txt are: " + fromUtf8(contents));
    } else {
        let w = container.write_object("file.txt");
        write_stream.write(w, "Hello, world!");
        console.log("Wrote 'Hello, world!' to file.txt");
    }
}