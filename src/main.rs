
use colored::Colorize;
use clap::{arg, Command};
use std::{fs, io::Read};
use sha3::{hash, hash_string};

struct TestState {
    test_runs: usize,
    test_successes: usize
}

impl Default for TestState {
    fn default () -> TestState {
        TestState {
            test_runs: 0,
            test_successes: 0
        }
    }
}

impl TestState {
    fn success(&mut self) {
        self.test_runs += 1;
        self.test_successes += 1;
    }

    fn failure(&mut self) {
        self.test_runs += 1;
    }

    fn all_passed(&mut self) -> bool {
        return  self.test_runs == self.test_successes;
    }
}

fn 
run_test(n: usize, test_state: &mut TestState, input: &str, expected: &str) {
    if hash_string(input, n).eq(expected) {
        print!("{}", "[OK]".bold().green());
        println!(" SHA3-{} (\"{}\")", n, input);
        test_state.success();
    } else {
        print!("{}", "[FAILED]".bold().red());
        println!(" SHA3-{} (\"{}\") != {}", n, input, expected);
        test_state.failure();
    }
}

fn
tests() {
    let mut test_state = TestState::default();

    run_test(224, &mut test_state, "", "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7");
    run_test(224, &mut test_state, "abcde", "6acfaab70afd8439cea3616b41088bd81c939b272548f6409cf30e57");
    run_test(224, &mut test_state, "6acfaab70afd8439cea3616b41088bd81c939b272548f6409cf30e57", 
      "bd9c9f3ffa82a4492078a815b4d4d8f534be0a0144c619e391a299e6");

    run_test(256, &mut test_state, "", "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a");
    run_test(256, &mut test_state, "abcde", "d716ec61e18904a8f58679b71cb065d4d5db72e0e0c3f155a4feff7add0e58eb");
    run_test(256, &mut test_state, "d716ec61e18904a8f58679b71cb065d4d5db72e0e0c3f155a4feff7add0e58eb", 
        "58e437cd7fb13bc1b9537cd02d2bd7bfab5f5c66e604d32757a2de74e2a4e058");

    run_test(384, &mut test_state, "", "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004");
    run_test(384, &mut test_state, "abcde", "348494236b82edda7602c78ba67fc3838e427c63c23e2c9d9aa5ea6354218a3c2ca564679acabf3ac6bf5378047691c4");
    run_test(384, &mut test_state, "348494236b82edda7602c78ba67fc3838e427c63c23e2c9d9aa5ea6354218a3c2ca564679acabf3ac6bf5378047691c4", 
        "e69c1eb7639e1fcb1824c50f5c128a460649eea67ee00e73054434dd9cce095828223645a1a63047ecb4ad93e88ea39b");

    run_test(512, &mut test_state, "", "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26");
    run_test(512, &mut test_state, "abcde", "1d7c3aa6ee17da5f4aeb78be968aa38476dbee54842e1ae2856f4c9a5cd04d45dc75c2902182b07c130ed582d476995b502b8777ccf69f60574471600386639b");
    run_test(512, &mut test_state, "1d7c3aa6ee17da5f4aeb78be968aa38476dbee54842e1ae2856f4c9a5cd04d45dc75c2902182b07c130ed582d476995b502b8777ccf69f60574471600386639b", 
"31f82868746cf95d8fdeccd6f91fd6d998297eba09c87da23d8e174ba2a51acda1c26a5a5c601c0f1292ed9585a706780b77cfbfa2d56cc168743f4cd30ffea3");

    if test_state.all_passed() {
        println!("All {} tests completed successfully!", test_state.test_runs);
    } else {
        println!("{}[{} / {}] passed", "Test failures: ".bold().red(), test_state.test_successes, test_state.test_runs);
    }

}

fn 
main () {
    let matches = Command::new("sha3")
        .version("0.1")
        .about("SHA3-224, 256, 384 and 512")
        .arg(arg!(--path <VALUE>).required(false))
        .arg(arg!(--string <VALUE>).required(false))
        .arg(arg!(--algo <VALUE>).required(false))
        .arg(arg!(--test).required(false))
        .get_matches();

    let string = matches.get_one::<String>("string");
    let path = matches.get_one::<String>("path");
    let algo = matches.get_one::<String>("algo");
    let test = matches.get_one::<bool>("test");

    let n = match algo.as_deref() {
        None => {
            if test.is_none() || !test.unwrap() {
                println!("no algorithim specified; assuming SHA3-256");
            }
            256
        },
        Some(s) => match s.as_str() {
            "224" => 224,
            "256" => 256,
            "384" => 384,
            "512" => 512,
            _ => {
                println!("unsupported algorithm; provide '224', '256', '384' or '512'");
                return;
            }
        },
    };

    match (string, path, test) {
        (Some(&ref text), None, Some(false)) => {
            let digest = hash_string(&text, n);
            println!("{}", digest);
        },
        (None, Some(f), Some(false)) => {
            let mut file_data: Vec<u8> = Vec::new();
            let mut file = fs::File::open(f).expect("unable to open file");

            file.read_to_end(&mut file_data).expect("unable to read data");

            let digest = hash(&mut file_data, n);
            println!("{}", digest);
        },
        (None, None, Some(true)) => {
            tests();
        }
        _ => {
            println!("no text provided!");
        }
    }
}