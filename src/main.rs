
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::seq::index;
use std::sync::{Arc, Mutex};



struct server{

    ip: String,
    state: bool,
}

fn main() -> std::io::Result<()>{
    {

        // The client should be aware of servers that are up, from the leader 
      let mut temp: [server; 3] = [server { ip: String::from("127"), state: true }, server { ip: "192.168.1.3:5960".to_string(), state: true }, server { ip: "192.168.1.3:5960".to_string(), state: true },];
      let serverIps = Arc::new(Mutex::new(temp));

        let serverIps1 = Arc::clone(&serverIps);
        let handler = thread::spawn( move || {
        let socket = UdpSocket::bind("192.168.1.3:5966").unwrap();

            loop {
                println!("Sending message to other servers");
                let mut rng = rand::thread_rng();
                let n = rng.gen_range(0, 6);
                println!("Random number: {n}");

                // forwarding the request in a random faashion to one of the servers 
                if n == 5{
                    let mut num = serverIps1.lock().unwrap();
                    num[0].state = false;
                        // I am server 0
                        socket.send_to(b"0", &num[1].ip).unwrap();
                        socket.send_to(b"0", &num[2].ip).unwrap();

                        std::mem::drop(num);
                } 
    
            }
            
            });

            let serverIps2 = Arc::clone(&serverIps);
            // thread to receive messages from other servers
            // dedicated thread to avoid being blocked by recv_from
            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind("192.168.1.3:5960").unwrap();
        
                loop {
                        println!("Recieving messages from other servers");
                        let mut buf = [0; 1]; // buffer for recieving 


                        // blocked till Recieving a message from any of the other servers 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        let mut num = serverIps2.lock().unwrap();
                        if &buf == b"0"{
                       
                            num[0].state = false;
                        }  else if &buf == b"1"{
                      
                            num[1].state = false;
                        } else if &buf == b"2"{
                          
                            num[2].state = false;
                        }
                        std::mem::drop(num);

                        println!("Message Recieved!");

                        println!("From: {:?}", src);
                        //print the received data as a string 

                        println!("Message: {}", String::from_utf8_lossy(&buf));
                 
                    }
                        
                });


        let socket = UdpSocket::bind("192.168.1.3:5959").unwrap();

        // main thread to comunicate with the agents
        loop {
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            thread::sleep(Duration::from_millis(100));

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            socket.send_to(buf, &src).unwrap();
        }


        handler.join().unwrap();
        handler1.join().unwrap();
    }

    Ok(())
    

}

