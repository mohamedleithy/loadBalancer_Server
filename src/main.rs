
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() -> std::io::Result<()>{
    {


        // thread to send messages to other servers
        let handler = thread::spawn(move || {
        let socket = UdpSocket::bind("172.20.10.13:5966").unwrap();

            while true {
                println!("Sending message to other servers");
                let mut rng = rand::thread_rng();
                let n = rng.gen_range(0, 6);
                println!("Random number: {n}");
                
                if n == 5{
                    // socket.send_to(b"Server 1 down", "172.20.10.13:34255").unwrap();
                    socket.send_to(b"Server 1 down", "172.20.10.13:5960").unwrap();
                }

               thread::sleep(Duration::from_millis(1000));
    
            }
            
            });


            // thread to receive messages from other servers
            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind("172.20.10.13:5960").unwrap();
        
                while true {
                        println!("Recieving messages from other servers");

                        let mut buf = [0; 30];
                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        println!("Message Recieved!");
                        // println!("Message: {:?}", &buf[..amt]);
                        println!("From: {:?}", src);
                        //print the received data as a string
                        println!("Message: {}", String::from_utf8_lossy(&buf));



                        thread::sleep(Duration::from_millis(1000));
                 
                    }
                        
                });


        let socket = UdpSocket::bind("172.20.10.13:5959").unwrap();

        // main thread to comunicate with the clients
        while true {
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
