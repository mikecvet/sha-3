# SHA-3

This is a readable and highly annotated implementation of the set of SHA-3 cryptographic digest algorithms, implemented in Rust.

There are a few simple arguments to the program:

    ~/code/sha-3 ~>> cargo build --release
    Finished release [optimized] target(s) in 0.03s

    ~/code/sha-3 ~>> ./target/release/sha3 --test
    [OK] SHA3-224 ("")
    [OK] SHA3-224 ("abcde")
    [OK] SHA3-224 ("6acfaab70afd8439cea3616b41088bd81c939b272548f6409cf30e57")
    [OK] SHA3-256 ("")
    [OK] SHA3-256 ("abcde")
    [OK] SHA3-256 ("d716ec61e18904a8f58679b71cb065d4d5db72e0e0c3f155a4feff7add0e58eb")
    [OK] SHA3-384 ("")
    [OK] SHA3-384 ("abcde")
    [OK] SHA3-384 ("348494236b82edda7602c78ba67fc3838e427c63c23e2c9d9aa5ea6354218a3c2ca564679acabf3ac6bf5378047691c4")
    [OK] SHA3-512 ("")
    [OK] SHA3-512 ("abcde")
    [OK] SHA3-512 ("1d7c3aa6ee17da5f4aeb78be968aa38476dbee54842e1ae2856f4c9a5cd04d45dc75c2902182b07c130ed582d476995b502b8777ccf69f60574471600386639b")

    ~/code/sha-3 ~>> ./target/release/sha3 --string abcde
    no algorithim specified; assuming SHA3-256
    d716ec61e18904a8f58679b71cb065d4d5db72e0e0c3f155a4feff7add0e58eb

    ~/code/sha-3 ~>> echo -n abcde > input_file.txt
    ~/code/sha-3 ~>> ./target/release/sha3 --path input_file.txt --algo 256
    d716ec61e18904a8f58679b71cb065d4d5db72e0e0c3f155a4feff7add0e58eb

    ~/code/sha-3 ~>> ./target/release/sha3 --path input_file.txt --algo 512
    1d7c3aa6ee17da5f4aeb78be968aa38476dbee54842e1ae2856f4c9a5cd04d45dc75c2902182b07c130ed582d476995b502b8777ccf69f60574471600386639b

I tested the performance of this code on the [2006 English Wikipedia Corpus](http://mattmahoney.net/dc/textdata.html), whose size comes in around ~954Mb.

    ~/code/sha-3 ~>> time ./target/release/sha3 --path ~/Downloads/wiki/enwik9 --algo 512
    5675affe508b9bbaf2eee12f0e8f7f6aed51fe8428b71e6a64b067f6492e40fd7160b3b19eff2dff3056487e3aeccd3c94f5cc81732c5cb9ced7641978eef2f4

    real	0m3.548s
    user	0m3.321s
    sys	0m0.225s 
