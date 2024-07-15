I am having an issue with CW-storage-plus. (thanks to Eris/Phil for pointing it out)

I have managed to produce an example using the current versions of the packages which demonstrates it


I have a key which is a (u64,&str) ... and it works ok, until I put in a u64 > 128, and it then gives a UTF-8 error


1. how do I remedy this.

2. is there a way to do this without migrating the index in the existing smart contract


## error code returned
```
called `Result::unwrap()` on an `Err` value: InvalidUtf8 { msg: "invalid utf-8 sequence of 1 bytes from index 9" }
thread 'testing::tests::test_128' panicked at contracts/hub-tf/src/testing/tests.rs:2100:10:
```