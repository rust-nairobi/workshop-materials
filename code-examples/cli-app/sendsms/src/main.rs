extern crate clap;
extern crate africastalking_gateway;

use clap::{App, Arg};
use africastalking_gateway::{AfricasTalkingGateway, SMSMessage};

fn main() {
    let cli = App::new("sendsms")
        .version("0.0.1")
        .author("Matt Gathu")
        .about("send SMS on the terminal")
        .arg(Arg::with_name("to")
             .long("to")
             .short("t")
             .takes_value(true)
             .help("number to send sms to ex +254702006545")
             .required(true)
        )
        .arg(Arg::with_name("message")
             .long("message")
             .short("m")
             .takes_value(true)
             .required(true)
        );

    let matches =  cli.get_matches();
    let username = "sandbox";
    let apikey = "d2cb5a86944fd45f279e8dba28c693c65801ba6ea0840df1ac625f6e08b88a7e";
    let gateway = AfricasTalkingGateway::new(username, apikey, "sandbox");

    let to = matches.value_of("to").unwrap();
    let msg = matches.value_of("message").unwrap();

    let sms = SMSMessage {
        username: username.to_owned(),
        to: to.to_owned(),
        message: msg.to_owned(),
        ..Default::default()
    };

    println!("{}", gateway.send_message(sms).unwrap());
}
