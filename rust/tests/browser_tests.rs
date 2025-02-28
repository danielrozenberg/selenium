// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use assert_cmd::Command;
use rstest::rstest;
use std::env::consts::OS;

use crate::common::assert_output;

mod common;

#[rstest]
#[case("chrome", "chromedriver", "114", "114.0.5735.90")]
#[case("chrome", "chromedriver", "115", "115.0.5790")]
#[case("edge", "msedgedriver", "105", "105.0")]
#[case("edge", "msedgedriver", "106", "106.0")]
fn browser_version_test(
    #[case] browser: String,
    #[case] driver_name: String,
    #[case] browser_version: String,
    #[case] driver_version: String,
) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_selenium-manager"));
    cmd.args([
        "--browser",
        &browser,
        "--browser-version",
        &browser_version,
        "--debug",
        "--avoid-browser-download",
    ])
    .assert()
    .success()
    .code(0);

    let stdout = &cmd.unwrap().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    println!("{}", output);

    assert!(output.contains(&driver_name));
    if !browser_version.is_empty() && output.contains("cache") {
        assert!(output.contains(&driver_version));
    }
}

#[rstest]
#[case("wrong-browser", "", "", exitcode::DATAERR)]
#[case("chrome", "wrong-browser-version", "", exitcode::DATAERR)]
#[case("chrome", "", "wrong-driver-version", exitcode::DATAERR)]
#[case("firefox", "", "wrong-driver-version", exitcode::DATAERR)]
#[case("edge", "wrong-browser-version", "", exitcode::DATAERR)]
#[case("edge", "", "wrong-driver-version", exitcode::DATAERR)]
#[case("iexplorer", "", "wrong-driver-version", exitcode::DATAERR)]
fn wrong_parameters_test(
    #[case] browser: String,
    #[case] browser_version: String,
    #[case] driver_version: String,
    #[case] error_code: i32,
) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_selenium-manager"));
    let result = cmd
        .args([
            "--debug",
            "--browser",
            &browser,
            "--browser-version",
            &browser_version,
            "--driver-version",
            &driver_version,
        ])
        .assert()
        .try_success();

    assert_output(&mut cmd, result, vec!["in PATH"], error_code);
}

#[rstest]
#[case("chrome", "chromedriver")]
#[case("edge", "msedgedriver")]
#[case("firefox", "geckodriver")]
fn browser_beta_test(#[case] browser: String, #[case] driver_name: String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_selenium-manager"));
    let assert = cmd
        .args([
            "--browser",
            &browser,
            "--browser-version",
            "beta",
            "--avoid-browser-download",
            "--debug",
        ])
        .assert();

    let stdout = &assert.get_output().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    println!("output {}", output);
    assert!(output.contains(&driver_name) || output.contains("ERROR"));
}

#[rstest]
#[case(
    "windows",
    "chrome",
    r#"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe"#
)]
#[case(
    "windows",
    "chrome",
    r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#
)]
#[case("linux", "chrome", "/usr/bin/google-chrome")]
#[case(
    "macos",
    "chrome",
    r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome"#
)]
#[case(
    "macos",
    "chrome",
    r#"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"#
)]
fn browser_path_test(#[case] os: String, #[case] browser: String, #[case] browser_path: String) {
    if OS.eq(&os) {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_selenium-manager"));
        cmd.args(["--browser", &browser, "--browser-path", &browser_path])
            .assert()
            .success()
            .code(0);

        let stdout = &cmd.unwrap().stdout;
        let output = std::str::from_utf8(stdout).unwrap();
        println!("{}", output);
        assert!(!output.contains("WARN"));
    }
}
