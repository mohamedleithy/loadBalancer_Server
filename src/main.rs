
// importing the udp standard library

extern crate systemstat;
extern crate local_ip;

use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::seq::index;
use std::sync::{Arc, Mutex};
use systemstat::{System, Platform, saturating_sub_bytes};

use std::time;

// To store server status
struct server{

    ip: String,
    state: bool,
    temperature: u8,
}

fn main() -> std::io::Result<()>{
    {

        // The client should be aware of servers that are up, from the leader 
      let temp: [server; 3] = [server { ip: "172.20.10.6:2024".to_string(), state: true, temperature: 100}, server { ip: "172.20.10.6:2024".to_string(), state: true, temperature: 100 }, server { ip: "172.20.10.3:2024".to_string(), state: true, temperature: 100},];
      let serverInfo = Arc::new(Mutex::new(temp));
      let ip = local_ip::get().unwrap();
        // send to servers current temperature as a parameter to elect the server to go down
        let serverInfo1 = Arc::clone(&serverInfo);
        let handler = thread::spawn( move || {
        let sys = System::new();
        let socket = UdpSocket::bind(ip.to_string() + ":2025").unwrap();
        let serverOutThreadMsg = "serverOutThread::";
            loop {

                let cpu_temp = sys.cpu_temp().unwrap();
                
                println!("{} Sending message to other servers. Current temp: {}", serverOutThreadMsg, cpu_temp);
    
                let mut serverInfo11 = serverInfo1.lock().unwrap();
                let msg = format!("Temprature:{}", cpu_temp);
                let mut buffer = msg.as_bytes();


                for server in serverInfo11.iter(){
                    if server.ip != ip.to_string() + ":2025" {
                        socket.send_to(&buffer , &server.ip).unwrap();
                    }
                }
                println!("{}", serverInfo11[2].temperature.to_string());
                std::mem::drop(serverInfo11); 

            // every one minute send your current temperature to the neighboring servers
            let one_minute = time::Duration::from_millis(60000);
            thread::sleep(one_minute);
            
    
            }
            
            });


            let serverInfo2 = Arc::clone(&serverInfo);
            // thread to receive messages from other servers, to update table
            // dedicated thread to avoid being blocked by recv_from

            
            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind(ip.to_string()+":2024").unwrap();
                let serverInThreadMsg = "serverInThread::";
                loop {
                    println!("{} Receiving message from other servers", serverInThreadMsg);
                        
                        let mut buf = [0; 100]; // buffer for recieving 


                        // blocked till Recieving a message from any of the other servers 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        let mut serverInfo22 = serverInfo2.lock().unwrap();
            
                        

                        println!("Message Recieved!");

                        println!("From: {:?}", src);
                        //print the received data as a string 
                        let t = &buf[11..15];
                        println!("Message: {}", String::from_utf8_lossy(&t));
                       

                        // update the corresponding server in the serverInfo list

                        for server in serverInfo22.iter_mut(){
                            if src.to_string() == (server.ip.to_string()+ ":2025")  {
                                println!("hey"); 
                                server.temperature = String::from_utf8((&t).to_vec()).unwrap().parse::<u8>().unwrap();
                            }

                        }

                        std::mem::drop(serverInfo22);

                        
                       
                    }
                        
                });


        let socket = UdpSocket::bind(ip.to_string()+":2023").unwrap();
        let agentsThreadMsg = "agentsThread::";
        // main thread to comunicate with the agents to perform main server functionality (reverse word)
        loop {
            println!("{} Recieving messages from agents", agentsThreadMsg);
            let mut buf = [0; 60];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            thread::sleep(Duration::from_millis(100));

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            socket.send_to(buf, ip.to_string()+":2021").unwrap();
        }


        handler.join().unwrap();
        handler1.join().unwrap();
    }

    Ok(())
    

}

