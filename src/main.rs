extern crate spring_dvs;

use std::net::UdpSocket;
use std::io::{ErrorKind};
use std::time::Duration;
use std::env;
use std::io;
use std::io::prelude::*;

//use spring_dvs::protocol::Port;

enum ArgState {
	None, Target, Protocol, Port
}
#[derive(Debug)]
enum Protocol {
	Dvsp, Http,
}

enum CommandMode {
	
}

struct Config {
	target: String,
	protocol: Protocol,
	port: u32,
}

fn main() {
    
    let mut cfg = Config {
    	target: String::from("127.0.0.1"),
    	protocol: Protocol::Dvsp,
    	port: 55301
    	 };
    let mut state: ArgState = ArgState::None;
    
    let mut msg = String::new();
    let mut args = env::args();
    args.next();
    for a in args {
        
        match a.as_ref() {
            
            "-t" => { state = ArgState::Target },
            "-s" => { state = ArgState::Protocol },
            "-p" => { state = ArgState::Port },
            _ => {
            	match state {
            		ArgState::Target => {
            			cfg.target = String::from(a)
            		},
            		ArgState::Protocol => {
            			cfg.protocol = match a.as_ref() {
            				"dvsp" => Protocol::Dvsp,
            				"http" => Protocol::Http,
            				_ => Protocol::Dvsp,
            			};
            		},
	          		ArgState::Port => {
						cfg.port = match a.parse::<u32>() {
							Ok(p) => p,
							Err(e) => panic!("{:?}", e)
						}
            		},
            		ArgState::None => {
	            		continue;
            		}
            	}
            	state = ArgState::None;
            }
        }
    }

	loop {
		
		io::stdout().write("$ ".as_bytes()).unwrap();
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut msg).unwrap();
		
		if msg == "exit\n" {
			println!("Exiting...");
			break;
		}
	    
    
	    match cfg.protocol {
		    Protocol::Dvsp => service_udp(&msg.trim(), &cfg),
		    Protocol::Http => println!("Unsupported protocol"),
	    }
	    
	    msg.clear();
    
	}

}

fn service_udp(msg: &str, cfg: &Config) {
		let address = format!("{}:{}", cfg.target, cfg.port);
		
		let socket = match UdpSocket::bind("0.0.0.0:0") {
				Ok(s) => s,
				Err(_) => panic!("Error binding socket")
		};
		
		match socket.set_read_timeout(Some(Duration::new(20,0))) { // 20 Second Timeout
			Ok(_) => { },
			_ => panic!("Error setting timeout")
		}
		
		match socket.send_to(msg.as_bytes(), address.as_str()) {
			Ok(_) =>{ },
			_ => panic!("Error Writing to socket")
		}
		
		let mut buf = [0;768];
		let (sz, _) = match socket.recv_from(&mut buf) {
			Ok(t) => t,
			Err(e) => {
				match e.kind() { 
					ErrorKind::TimedOut => panic!("Error timed out"),
					_ => panic!("Error reading from socket") 
				}
			} 

		};
		
		let mut v : Vec<u8> = Vec::new();
		v.extend_from_slice(&buf[0..sz]);
		let s = match String::from_utf8(v) {
			Ok(s) => s,
			Err(_) => panic!("Error on response conversion")
		};
		
		println!("{}", s);
}
