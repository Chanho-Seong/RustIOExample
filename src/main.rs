extern crate greprs;    // 라이브러리 크레이트(모듈)를 스코프 내로 가져오기

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();  // 여러 콜렉션에서 사용되는 함수 이므로, 타입을 명시해야 한다.

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Prooblem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);     // 디버그 형식자 :?

    println!("searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

