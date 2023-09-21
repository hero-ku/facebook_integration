use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, TcpListener};
use serde_json::Value;

pub mod api;
use api::Change;

static VERIFY_TOKEN: &str = "meathamhock";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        process_request(stream.unwrap());
    }
}

fn parse_request_line(line: &str) -> (&str, &str, HashMap<String, String>, &str) {
    let split = line.split(" ").collect::<Vec<&str>>();
    let mut arguments: HashMap<String, String> = HashMap::new();

    let position = match split[1].chars().position(|x| x == '?') {
        Some(pos) => pos,
        None => split[1].chars().count(),
    };

    if position < split[1].chars().count() {
        let mut arg_string = split[1].split_at(position + 1).1.chars();

        'outer: loop {
            let mut name = String::new();

            loop {
                let next = arg_string.next();

                match next {
                    Some('=') => break,
                    Some(char) => name += char.to_string().as_str(),
                    None => break 'outer,
                }
            }

            let mut param = String::new();

            loop {
                let next = arg_string.next();

                match next {
                    Some('&') | None => break,
                    Some(char) => param += char.to_string().as_str(),
                }
            }

            arguments.insert(name, param);
        }
    }

    (split[0], split[1].split_at(position).0, arguments, split[2])
}

fn process_request(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let mut lines = reader.lines();

    let request_line = lines.next().unwrap().unwrap();

    let iter = lines.skip_while(|x| {
        if let Ok(str) = x {
            !str.is_empty()
        } else {
            false
        }
    });

    let body = iter.map(|x| x.unwrap()).collect::<Vec<String>>().join("\n");

    println!("{request_line}");
    let (method, uri, args, version) = parse_request_line(&request_line);
    println!("{method} {uri}");

    let response = match (method, uri) {
        ("GET", "/webhook") if args.get("hub.mode") == Some(&"subscribe".to_string()) => Some(handle_verification(args)),
        ("POST", "/webhook") => { handle_event(args, body); None },
        _ => Some(String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"))
    };

    if let Some(response) = response {
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn handle_verification(args: HashMap<String, String>) -> String {
    if args.get("hub.verify_token") == Some(&VERIFY_TOKEN.to_string()) {
        match args.get("hub.challenge") {
            Some(challenge) => format!("HTTP/1.1 200 OK\r\n\r\n{challenge}"),
            None => String::from("HTTP/1.1 200 OK\r\n\r\nMissing challenge code!")
        }
    } else {
        String::from("HTTP/1.1 200 OK\r\n\r\nMissing or incorrect verification token!")
    }
}

fn handle_event(args: HashMap<String, String>, body: String) {
    let json: Result<Value, _> = serde_json::from_str(&body);
    if json.is_err() { return };

    println!("{json:#?}");

    let json = json.unwrap();

    if let Value::Object(object) = json {
        if object.contains_key()

        if let Some(Value::Array(entry)) = object.get("entry") {
            if let Value::Object(entry) = &entry[0] {
                println!("retrieved first entry");

                if let Some(Value::Array(changes)) = entry.get("changes") {
                    changes.iter().map(|x| )

                    println!("{:#?}", changes);
                }
            }
        }
    }
}