function _start() {
    let dl = distributed_locking.open("my-etcd-server");
    console.log("(1) trying to acquire a lock with 5s time to live");
    let now = new Date();
    let _lock_with_time_to_live = distributed_locking.lock_with_time_to_live(dl, "lock1", 5);
    console.log("(2) managed to acquire lock after " + (new Date() - now) + "ms" + ", this lock will be unlocked after 5s");

    console.log("(3) trying to acquire a lock with no specific time to live");
    now = new Date();
    let lock_with_no_time_to_live = distributed_locking.lock(dl, "lock2");
    console.log("(4) managed to acquire lock after " + (new Date() - now) + "ms");
    console.log("(5) pretend we are doing work by sleeping for 10s...");
    sleep(10000);
    console.log("(6) unlocked the lock we just acquired!");
    distributed_locking.unlock(dl, fromUtf8(lock_with_no_time_to_live));
}