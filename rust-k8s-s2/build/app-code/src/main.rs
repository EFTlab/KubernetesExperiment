#[macro_use] extern crate nickel;

use nickel::Nickel;
use core::time;
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::sync::Mutex;
use std::{thread};
use once_cell::sync::Lazy;
use chrono;

static INFO: Lazy<Mutex<String>> = Lazy::new(||Mutex::new(String::from("")));

fn get_info_text() -> String {
  let info = (*INFO).lock();
  let mut output = String::from("");
  match info {
    Err(_) => output.push_str(format!("Failed to fetch current log: {:?}", info).as_str()),
    Ok(_)  => output.push_str(format!("Current log is: {:?}", info).as_str())
  }
  return output
}
#[tokio::main]
async fn listen_for_web_status_async() {
  let mut server = Nickel::new();
  server.utilize(router! {
      get "**" => |_req, _res| {
        get_info_text()
      }
  });

  let result = server.listen("[::]:3200").await;
  match result {
    Err(_) => println!("Bad thing happened {:?}", result),
    Ok(_)  => println!("Good thing happened")
  }
}

fn listen_for_web_status() {
  listen_for_web_status_async()
}

fn listen_for_incoming_inner() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let listener = TcpListener::bind("[::]:3201")?;

  // accept connections and process them serially
  for stream in listener.incoming() {
    let my_result = handle_client(stream?);
    if my_result.is_err() {
      return my_result;
    }
  }

  Ok(())
}

fn listen_for_incoming() {
  let result = listen_for_incoming_inner();
  match result {
    Err(_) => println!("Bad thing happened incoming {:?}", result),
    Ok(_)  => println!("Good thing happened incoming")
  }
}

fn handle_client(mut stream: TcpStream) -> std::result::Result<(), Box<dyn std::error::Error>>  {
  let buf_reader = BufReader::new(&mut stream);
  let my_info: Vec<_> = buf_reader
      .lines()
      .map(|result| result.unwrap())
      .take_while(|line| !line.is_empty())
      .collect();

  (*INFO).lock()?.push_str(my_info.join("<br />").as_str());

  Ok(())
}

fn make_outgoing() {
  let four_seconds = time::Duration::from_secs(10);
  loop {
    {
      let stream_cxn_result = TcpStream::connect("rust-k8s-entrypoint-svc-s1:3101");
      match stream_cxn_result {
        Err(_) => println!("Couldn't connect to the remote server..."),
        Ok(_)  => write_to_outgoing(stream_cxn_result.unwrap())
      }
    }
    thread::sleep(four_seconds);
  }

}

fn write_to_outgoing(mut stream: TcpStream) {
  println!("Connected to the remote server!");
  let time_now = format!("{:?}", chrono::offset::Local::now());
  let my_string = format!("Hello {time_now} Dave!\n\n");
  let did_write_ok = stream.write(my_string.as_bytes());
  match did_write_ok {
    Err(_) => println!("Failed to write bytes {:?}", did_write_ok),
    Ok(_)  => ()
  }
  let did_flush_ok = stream.flush();
  match did_flush_ok {
    Err(_) => println!("Failed to flush bytes {:?}", did_flush_ok),
    Ok(_)  => ()
  }
  let did_shutdown_ok = stream.shutdown(std::net::Shutdown::Both);
  match did_shutdown_ok {
    Err(_) => println!("Failed to shutdown ok {:?}", did_shutdown_ok),
    Ok(_)  => ()
  }
}

fn main() {
  let t1 = thread::spawn(listen_for_web_status);
  let t2 = thread::spawn(listen_for_incoming);
  let t3 = thread::spawn(make_outgoing);

  t1.join().expect("The t1 thread has panicked");
  t2.join().expect("The t2 thread has panicked");
  t3.join().expect("The t3 thread has panicked");
}
