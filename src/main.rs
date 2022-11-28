
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
use std::vec;

use std::time;

// To store server status
struct server{
    ip: String,
    state: bool,
    cpu_score: u8,
}

fn main() -> std::io::Result<()>{
    {
    
      let ip = local_ip::get().unwrap();
      let tempAgents: Vec<server> = vec![]; 
      let tempServers: [server; 3] = [server { ip: ip.to_string(), state: true, cpu_score: 100}, server { ip: "192.168.8.106".to_string(), state: true, cpu_score: 13}, server { ip: "192.168.8.116".to_string(), state: true, cpu_score: 13},];
    
      let agents = Arc::new(Mutex::new(tempAgents));
      let serverInfo = Arc::new(Mutex::new(tempServers));
      
        // send to servers current temperature as a parameter to elect the server to go down


        // communicate with agents to inform them if this server is going down 
        // along with sending other servers current temp for update 

        let serverInfo1 = Arc::clone(&serverInfo);
        let handler = thread::spawn( move || {
        let sys = System::new();
        let socket = UdpSocket::bind(ip.to_string() + ":2025").unwrap();
        let serverOutThreadMsg = "serverOutThread::";
            loop {

                let cpu_temp = sys.cpu_temp().unwrap();
                let mem = sys.memory().unwrap();
                let mem_used = saturating_sub_bytes(mem.total ,mem.free);

                let mem_used =mem_used.to_string();
                
                let mut serverInfo11 = serverInfo1.lock().unwrap();

                // casting ByteSize to int
                let mem_used: Vec<&str> = mem_used.split(".").collect(); 
                let mem_used = mem_used[0].parse::<u8>().unwrap(); 

                let cpu_temp = cpu_temp.to_string();
                let cpu_temp: Vec<&str> = cpu_temp.split(".").collect(); 
                let cpu_temp = cpu_temp[0].parse::<u8>().unwrap(); 

                let score = cpu_temp + mem_used;
                let msg = format!("{}",score );
                println!("Score: {}", msg);
                let mut buffer = msg.as_bytes();


                for server in serverInfo11.iter(){
                        let a = format!("{}{}", server.ip, ":2024");
                        socket.send_to(&buffer , &a).unwrap();
                }

                std::mem::drop(serverInfo11); 

            // every one minute send your current temperature to the neighboring servers
           
            let one_minute = time::Duration::from_millis(60000);
            thread::sleep(one_minute);
            
            }
            
            });


            let serverInfo2 = Arc::clone(&serverInfo);
            // thread to receive messages from other servers, to update table
            // dedicated thread to avoid being blocked by recv_from

            let agents3 = Arc::clone(&agents);
            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind(ip.to_string()+":2024").unwrap();
                let serverInThreadMsg = "serverInThread::";
                loop {
                    println!("{} Receiving message from other servers", serverInThreadMsg);
                        
                        let mut buf = [0; 4]; // buffer for recieving 


                        // blocked till Recieving a message from any of the other servers 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        let mut serverInfo22 = serverInfo2.lock().unwrap();
            
                        

                        println!("Message Recieved!");

                        println!("From: {:?}", src);

                        //print the received data as a string 
                        println!("Message: {}", String::from_utf8_lossy(&buf));
                       

                        // update the corresponding server in the serverInfo list

                        for server in serverInfo22.iter_mut(){
                            if src.to_string() == (server.ip.to_string()+ ":2025")  {  
                                let val1 = (buf[0] - 48)*10; 
                                let val2 = buf[1] -48;
                                server.cpu_score = val1+ val2;
                            }

                        }
                        let mut agents33 = agents3.lock().unwrap();
                        
                        let mut myScore = 0;
                        let mut max  = 0;  

                        for server in serverInfo22.iter_mut(){
                            if(ip.to_string() == server.ip){
                                myScore = server.cpu_score;
                            }
                            if(server.cpu_score>max){
                                    max = server.cpu_score;
                            }
                        }
                        
                        if(myScore==max){
                            for agent in agents33.iter_mut(){

                                let adr = format!("{}{}", agent.ip, ":2022");
                                socket.send_to(b"0" , adr).unwrap();
    
                            }
                        }
                        


                        

                     
                    

                        std::mem::drop(agents33);
                        std::mem::drop(serverInfo22);

                        
                       
                    }
                        
                });

                // this thread is responsible to keep track of running agents 

                let agents2 = Arc::clone(&agents);

                let handler2 = thread::spawn(move || -> ! {
                    
                    let socket = UdpSocket::bind(ip.to_string()+":2030").unwrap();
                    let updatingAgentsThreadMsg = "UpdatingAgentsThread::";
                    loop {

                        // blocks until any of the agents informs the server it's up or down (checks based on buf)
                        let mut buf = [0; 100]; // buffer for recieving 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        

                        let src1 = src.ip().to_string(); 
                        let src1: Vec<&str> = src1.split(":").collect(); 
                        let src1 = src1[0]; 

                        let msg = String::from_utf8((&buf).to_vec()).unwrap();
                        let msg = msg.trim_matches(char::from(0));
                        
                        let mut agents22 = agents2.lock().unwrap();
                        if msg == "1" {
                             
                        agents22.push(server{ip: src1.to_string(), state: true, cpu_score: 0});

                        }else if msg == "0"{

                            println!("removing agent with ip: {}", src);
                            let size = agents22.len();
                            for i in 0..(size) {

                                // remove the agent from the active agents list 

                                if agents22[i].ip ==  src1.to_string() {
                                    agents22.remove(i); 
                                }
                            }

                        }



                        println!("{} Active agents are: \n", updatingAgentsThreadMsg); 

                        for agent in agents22.iter() {
                            println!("{}{}", updatingAgentsThreadMsg , agent.ip); 
                        }

                        std::mem::drop(agents22);
                   
                        }
                            
                    });







                    

        let socket = UdpSocket::bind(ip.to_string()+":2023").unwrap();
        let agentsThreadMsg = "agentsThread::";

        // main thread to comunicate with the agents to perform main server functionality (reverse word)
        loop {
            println!("{} Recieving messages from agents", agentsThreadMsg);
            let mut buf = [0; 60];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
       

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();

            let src1 = src.ip().to_string(); 
            let src1: Vec<&str> = src1.split(":").collect(); 
            let src1 = src1[0]; 
            let src1  = format!("{}{}", src1.to_string(), ":2021");
            socket.send_to(buf, src1).unwrap();
        }


        handler.join().unwrap();
        handler2.join().unwrap();
        handler1.join().unwrap();
    }

    Ok(())
    

}

