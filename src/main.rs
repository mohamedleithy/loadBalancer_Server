
// importing the udp standard library

extern crate systemstat;

use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::seq::index;
use std::sync::{Arc, Mutex};

use std::time;


struct server{

    ip: String,
    state: bool,
}

fn main() -> std::io::Result<()>{
    {

        // The client should be aware of servers that are up, from the leader 
      let mut temp: [server; 3] = [server { ip: "172.20.10.6:2024".to_string(), state: true }, server { ip: "172.20.10.6:2024".to_string(), state: true }, server { ip: "192.168.1.3:2024".to_string(), state: true },];
      let serverIps = Arc::new(Mutex::new(temp));

        let serverIps1 = Arc::clone(&serverIps);


        // LEITHY PC -> INDEX 0 
        // MARK PC -> INDEX  1 
        // THESIS PC -> INDEX 2

        
        // send to servers current memory usage as a parameter to elect the server to go down
        let handler = thread::spawn( move || {
        let socket = UdpSocket::bind("172.20.10.3:2025").unwrap();
        let serverOutThreadMsg = "serverOutThread::";
            loop {
                println!("{} Sending message to other servers", serverOutThreadMsg);
              
            
                    let mut num = serverIps1.lock().unwrap();
                    num[0].state = false;
                        // I am server 0
                        socket.send_to(b"0", &num[1].ip).unwrap();
                        // socket.send_to(b"0", &num[2].ip).unwrap();

                        std::mem::drop(num); 

            // every one minute send your current memory usage to the neighboring servers
            let one_minute = time::Duration::from_millis(60000);
            thread::sleep(one_minute);
    
            }
            
            });

            let serverIps2 = Arc::clone(&serverIps);
            // thread to receive messages from other servers, to update table
            // dedicated thread to avoid being blocked by recv_from

            
            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind("172.20.10.3:2024").unwrap();
                let serverInThreadMsg = "serverInThread::";
                loop {
                    println!("{} Receiving message from other servers", serverInThreadMsg);
                        
                        let mut buf = [0; 1]; // buffer for recieving 


                        // blocked till Recieving a message from any of the other servers 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        let mut num = serverIps2.lock().unwrap();
                        if &buf == b"0"{
                            println!("Server 0 Dropped", serverInThreadMsg);
                            num[0].state = false;
                        }  else if &buf == b"1"{
                            println!("Server 1 Dropped", serverInThreadMsg);
                            num[1].state = false;
                        } else if &buf == b"2"{
                            println!("Server 2 Dropped", serverInThreadMsg);
                            num[2].state = false;
                        }
                        std::mem::drop(num);

                        println!("Message Recieved!");

                        println!("From: {:?}", src);
                        //print the received data as a string 

                        println!("Message: {}", String::from_utf8_lossy(&buf));
                 
                    }
                        
                });


        let socket = UdpSocket::bind("172.20.10.3:2023").unwrap();
        let agentsThreadMsg = "agentsThread::";
        // main thread to comunicate with the agents to perform main server functionality (reverse word)
        loop {
            println!("{} Recieving messages from agents", agentsThreadMsg);
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            thread::sleep(Duration::from_millis(100));

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            socket.send_to(buf, "10.40.44.255:2021").unwrap();
        }


        handler.join().unwrap();
        handler1.join().unwrap();
    }

    Ok(())
    

}

