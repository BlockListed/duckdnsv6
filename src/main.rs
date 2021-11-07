pub mod config;
pub mod errorhandler;
use config::*;

use colorful::{Color, Colorful};

use isahc::{config::Configurable, Request, prelude::*};
use isahc::config::IpVersion;

use scraper::{Html, Selector};

fn main() -> Result<(), std::io::Error> {
    update()?;
    Ok(())
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

    let mut raw_ip: String;

    raw_ip = parsed_html.select(&selector).next().unwrap().inner_html();

    let ip: String;

    raw_ip.remove(0);
    raw_ip.pop();

    if !configuration.fritzbox {
        ip = raw_ip;
    } else {
        let segments: Vec<&str> = raw_ip.split(':').collect();
        let ip_start: String = segments[..4].join(":");

        ip = format!("{}:{}", ip_start, configuration.interfaceid.unwrap());
    }

    let uri = format!("https://www.duckdns.org/update?domains={}&token={}&ipv6={}", configuration.domain, configuration.token, ip);

    send_debug!(format!("Duckdns URI: {}", uri));

    let mut duckdnsresponse = isahc::get(uri).unwrap();

    send_debug!(format!("Duckdns Response: {}", duckdnsresponse.text().unwrap()));

    Ok(())
}