use std::io::{Read, Result, Write};
use std::path::PathBuf;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct DfsClientConn {
    conn_id: u32,
    conn: TcpStream,
    buff: Vec<u8>,
}

impl DfsClientConn {
    fn handle_conn(&mut self) -> i32 {
        1
    }
}

pub struct DfsHost {
    next_client_id: u32,
    root_path: PathBuf,
    clients: Vec<Arc<Mutex<DfsClientConn>>>,
}

impl DfsHost {
    pub fn new(root_path: PathBuf) -> Self {
        DfsHost {
            next_client_id: 1,
            root_path,
            clients: Vec::new(),
        }
    }

    pub fn start_listening(&mut self, bind_address: &str) -> Result<()> {
        // Bind a TcpListener to listen on bind_address
        let listener = TcpListener::bind(bind_address).unwrap();
        
        println!("Listening for incoming connections on {bind_address}...");

        for stream in listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    println!("Accepted a new connection from: {:?}", tcp_stream.peer_addr());
                    // Create a new DfsClientConn instance for each connection and store it
                    let new_client = Arc::new(Mutex::new(
                        DfsClientConn {
                            // Initialize fields for DfsClientConn as needed
                            conn_id: self.next_client_id,
                            conn: tcp_stream,
                            buff: vec![0u8; 1024],
                        }
                    ));
                    self.next_client_id += 1;
                    self.clients.push(Arc::clone(&new_client));
                    let thread_client = Arc::clone(&new_client);
                    // Distribute to handle thread
                    let handle_thread = thread::spawn({
                        move || { 
                            let mut thread_client = thread_client.lock().unwrap();
                            thread_client.handle_conn();
                        }
                    });
                    handle_thread.join().expect(&format!("Thread for handling client failed."));
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        
        Ok(())
    }

}