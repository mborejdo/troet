#[macro_use]
extern crate clap;
extern crate winrt_notification;
use winrt_notification::{
    Duration,
    Sound,
    Toast,
};
use std::io::{self, BufRead};
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
                            .help("Message body")
                            .multiple(true)
                            .required(true))
                            
                    )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("send") {

        let summary = matches.value_of("summary").unwrap();
        // let body = matches.values_of("body").unwrap().collect::<Vec<&str>>().join(" ");
        let stdin = io::stdin();
        let body = stdin.lock().lines().next().unwrap().unwrap();

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(summary)
            .text1(&body)
            .sound(Some(Sound::SMS))
            .duration(Duration::Short)
            .show()
            .expect("unable to send notification");
        std::thread::sleep(std::time::Duration::new(2, 0));

    }
}