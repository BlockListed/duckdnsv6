#![warn(clippy::pedantic)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod config;
pub mod errorhandler;
use config::{Configuration, get_configuration};

use std::thread::sleep;
use std::time::Duration;

use colorful::{Color, Colorful};

use isahc::{config::Configurable, Request, prelude::*};
use isahc::config::IpVersion;

use scraper::{Html, Selector};

fn main() -> Result<(), std::io::Error> {
    #[cfg(all(not(feature = "once_run"), feature = "loop_run"))]
    loop_run()?;
    #[cfg(feature = "once_run")]
    update()?;
    Ok(())
}

fn loop_run() -> Result<(), isahc::Error> {
    loop {
        update()?;
        sleep(Duration::from_secs(5 * 60));
    }
}

fn update() -> Result<(), isahc::Error> {
    let configuration: Configuration = handle_error!(get_configuration());
    let rawhtml: String = Request::get(configuration.ipv6lookup)
    .ip_version(IpVersion::V6)
    .body(())?
    .send()?
    .text()?;

    let parsed_html: Html = Html::parse_document(rawhtml.as_str());
    let selector = Selector::parse("span").unwrap();

    let mut ip = parsed_html.select(&selector).next().unwrap().inner_html();

    ip.remove(0);
    ip.pop();

    let uri = format!("https://www.duckdns.org/update?domains={}&token={}&ipv6={}", configuration.domain, configuration.token, ip);

    send_debug!(format!("Duckdns URI: {}", uri));

    let mut duckdnsresponse = isahc::get(uri).unwrap();

    send_debug!(format!("Duckdns Response: {}", duckdnsresponse.text().unwrap()));

    Ok(())
}