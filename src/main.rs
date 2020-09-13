#[macro_use]
extern crate clap;
extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("troet")
        .version(&crate_version!()[..])
        .author("Michael Borejdo <mib@electronic-minds.de>")
        .about("sending desktop notifications since 2020")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("send")
                    .about("Shows a notification")
                    .arg( Arg::with_name("summary")
                            .help("Title of the Notification.")
                            .required(true))
                    .arg( Arg::with_name("body")
                            .help("Message body"))
                    )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("send") {

        let summary = matches.value_of("summary").unwrap();
        let body = matches.value_of("body").unwrap();

        Toast::new("327f4cda-428e-414e-bc49-67a4789ecb92")
            .title(summary)
            .text1(body)
            .show()
            .expect("unable to send notification");
        std::thread::sleep(std::time::Duration::new(2, 0));

    }
}