use futures::prelude::*;
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use libp2p::{identity, Multiaddr, PeerId, ping};
use std::error::Error;
use std::net::UdpSocket;
use std::{str, vec};
use std::string;
use std::thread::sleep;
use std::time::{SystemTime, Duration,UNIX_EPOCH};


fn increment_computed(  number_of_computed_requests: &mut i32,client_ip :  &[u8], socket_to_client : & UdpSocket,port_number :  String) {
    

    *number_of_computed_requests+=1;
        // reply      
        let temp = client_ip.len();
        let res = String::from_utf8(client_ip[0..temp].to_vec()).expect("Found invalid UTF-8");
       //let temp1 = client_ip[1..temp].to_owned();
       let res = res.trim_matches(char::from(0));
        socket_to_client.connect(res).expect("couldn't connect to address");
        let first= "Hi";
        let concat_str = format!("{}{}",first, port_number);

        socket_to_client.send(concat_str.as_bytes());

        println!("computed is {}", *number_of_computed_requests);
        // socket_to_client.send(b"Hi");
        

} 
    

    //println!("nodeID {} index {} recieved {} computed {} roundtotal {}\n", node_id,round_robin_index,number_of_recieved_requests, number_of_computed_requests,roundtotal);





#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut roundtotal = 3;
    let  mut node_id =0 ;
    let  mut node_id2 =0 ;
    let  mut node_id3 =0 ;
    let  mut node_id4 =0 ;
    let  mut node_id5 =0 ;
    let  mut node_id6 =0 ;
    let  mut node_id8 =0 ;
    let  mut node_id7 =0 ;
    let  mut node_idcopy = 0;
    if let Some(node_id_s) = std::env::args().nth(1) {
        node_id = node_id_s.parse()?;
        println!("node_id {}", node_id_s)
    }

    node_idcopy = node_id;
    let node_id_copy2 =  node_id + 3;
    let port_id = node_id.to_string();
    let port_id2 = node_id_copy2.to_string();


    node_id+=800; //socket send for servers
    node_id2= node_id + 5; //socket recieve for servers
    node_id3= node_id + 10; //socket send for servers to clients
    node_id4= node_id + 15; //socket reciever for clients

    node_id5 = node_id + 20; //socket to send to Agent 1
    node_id6 =  node_id + 25; //socket to recieve from Agent 1

    node_id7 = node_id + 30; // socket to send to Agent 2
    node_id8 = node_id + 35; // docket to recieve from Agent 2
    let port:String = node_id.to_string();
    let port2: String = node_id2.to_string();
    let port3: String = node_id3.to_string();
    let port4: String = node_id4.to_string();

    

    let port5: String = node_id5.to_string();
    let port6: String = node_id6.to_string();
    let port7: String = node_id7.to_string();
    let port8: String = node_id8.to_string();




    let temp_string: String = "127.0.0.2:".to_string();

    let temp_string2: String = "A hello from server ".to_string();

    let server_send_agent1 = format!("{}{}", temp_string, port5);
    let server_rsv_agent1 = format!("{}{}", temp_string, port6);
    println!("rec on {}", server_rsv_agent1);
    let server_send_agent2 = format!("{}{}", temp_string, port7);
    let server_rsv_agent2 = format!("{}{}", temp_string, port8);

    //socket send and recieve for agent1
    let socket_send_agent1 = UdpSocket::bind(server_send_agent1).expect("couldn't connect to address 1");
    socket_send_agent1.connect("127.0.0.2:902");


    let socket_rsv_agent1 = UdpSocket::bind(server_rsv_agent1).expect("couldn't connect to address 1");

    //socket send and recieve for agent2

    let socket_send_agent2 =  UdpSocket::bind(server_send_agent2).expect("couldn't connect to address 1");
    socket_send_agent2.connect("127.0.0.2:912");


    let socket_rsv_agent2 = UdpSocket::bind(server_rsv_agent2).expect("couldn't connect to address 1");


    ////////////////////////

    let together3 = format!("{}{}", temp_string2, node_id);



    let together = format!("{}{}", temp_string, port);
    let together2 = format!("{}{}", temp_string, port2);
    let together_client = format!("{}{}", temp_string, port3);
    let together_client_server =  format!("{}{}", temp_string, port4);
    println!("id/port is {}", together);
    println!("id/port together 2 is {}", together2);
    let socket = UdpSocket::bind(together).expect("couldn't connect to address 1");

    let socket2 = UdpSocket::bind(together2).expect("couldn't connect to address");

    let socket_to_client = UdpSocket::bind(together_client).expect("couldn't connect to address");
    let socket_client_to_server = UdpSocket::bind(together_client_server).expect("couldn't connect to address");

    if node_id == 801 {
        socket.connect("127.0.0.2:807").expect("couldn't connect to address");
        println!("id/port is 12");
    } else if node_id == 802 {
        socket.connect("127.0.0.2:808").expect("couldn't connect to address");
        println!("id/port is 13");
    } else {
        socket.connect("127.0.0.2:806").expect("couldn't connect to address");
        println!("id/port is 111");
    }


    // let socket2 = UdpSocket::bind("127.0.0.2:802").await?;

   


    let mut round_robin_index =1;
    let mut number_of_computed_requests =0;
    let mut number_of_recieved_requests =0;

    let one_seconds= Duration::from_secs(1);
    
    let one_milli= Duration::from_millis(10);
    let seconds = Duration::from_secs(20);


    socket.set_nonblocking(true);
    socket2.set_nonblocking(true);
    
    socket_rsv_agent1.set_read_timeout(Some(one_milli)).expect("set_read_timeout call failed");
    socket_rsv_agent2.set_read_timeout(Some(one_milli)).expect("set_read_timeout call failed");


   

    let mut sleep_counter=0;
    let mut counter_warm = 0;
    loop {

    let k ;

    let mut buf = vec![0; 1024];
  //socket.send(together3.as_bytes());
    socket2.recv(&mut buf);
    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };


 


    
    for (i, c) in s.chars().enumerate() {
        // do something with character `c` and index `i`

            if(c == 'E' &&  i == 0){

                if(node_idcopy == 3)
                {
                    //println!("enterted node 3 turning it to 2");
                    roundtotal = 2;
                    node_idcopy = 2;
                    //round_robin_index = 2;
                }

            }else if(c == 'F' && i == 0){

                if(node_idcopy == 2)
                {
                    //println!("enterted the empty IF");
                    socket.send(b"E2");
                    roundtotal = 2;
                    node_idcopy = 1;
                }

                
            }else if(c == 'R' && i == 0){

                if(node_idcopy == 1)
                {
                   // println!("node woke up 2 saw this");
                    roundtotal = 3;
                    socket.send(b"R1");
                }

                if(node_idcopy == 2)
                {
                   // println!("node woke up 3 saw this");
                    roundtotal = 3;
                }
            }

         
       break;    
    }

    
    let mut buff = vec![0; 1024];
  //socket.send(together3.as_bytes());
    socket_rsv_agent1.recv(&mut buff);
    let t = match str::from_utf8(&buff) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    for (i, c) in t.chars().enumerate() {
        if ((c == '1' || c == '2' ||c == '3' ||c == '4' ||c == '5' ||c == '6' ||c == '7' ||c == '8' ||c == '9') && i == 0){
            increment_computed( &mut number_of_computed_requests,  t.as_bytes(),  & socket_to_client,  port4.to_owned());
        }
    }

    let mut bufff = vec![0; 1024];
  //socket.send(together3.as_bytes());
    socket_rsv_agent2.recv(&mut bufff);
    let U = match str::from_utf8(&bufff) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    for (i, c) in U.chars().enumerate() {
        if ((c == '1' || c == '2' ||c == '3' ||c == '4' ||c == '5' ||c == '6' ||c == '7' ||c == '8' ||c == '9') && i == 0){
            increment_computed( &mut number_of_computed_requests,  U.as_bytes(),  & socket_to_client,  port4.to_owned());
        }
    }

    
    if(node_idcopy == 1){

        
     

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) =>k = n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }

        counter_warm+=1;
        // if (k%10==0 ){
        // println!("time is {} ", k%90);
        // counter_warm = 0;
        // }

        if (k%60==0){
           sleep(one_seconds);
           println!("sleeping");
            socket.send(b"F2");
            socket_send_agent1.send(port_id.as_bytes());
            socket_send_agent2.send(port_id.as_bytes());
            sleep(seconds);
            println!("woke up");
            socket_send_agent1.send(port_id2.as_bytes());
            socket_send_agent2.send(port_id2.as_bytes());
           // println!("nodeID {} index {} recieved {} computed {} roundtotal {}\n", node_id,round_robin_index,number_of_recieved_requests, number_of_computed_requests,roundtotal);
            node_idcopy = 3;
            roundtotal = 3;
            println!("nodeID {} computed {}", node_idcopy, number_of_computed_requests);
            socket.send(b"R2");
        }
    }




      

       

        // // socket2.send(b"hello", ).await?;
        // socket2.recv(&mut buf).await?;
        // println!("Received bytes");
      
    }

}