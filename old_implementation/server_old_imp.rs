use futures::prelude::*;
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use libp2p::{identity, Multiaddr, PeerId, ping};
use std::error::Error;
use std::net::UdpSocket;
use std::str;
use std::thread::sleep;
use std::time::{SystemTime, Duration,UNIX_EPOCH};


fn increment_round_robin(  round_robin_index: &mut i32, number_of_computed_requests: &mut i32,  number_of_recieved_requests: &mut i32,node_id : &mut i32, roundtotal : &mut i32) {
        *number_of_recieved_requests+=1;
        if *round_robin_index == *node_id{
            *number_of_computed_requests+=1;
        } 
        if *round_robin_index == *roundtotal{
            *round_robin_index=0;
        } 

        println!("nodeID {} index {} recieved {} computed {} roundtotal {}\n", node_id,round_robin_index,number_of_recieved_requests, number_of_computed_requests,roundtotal);


        *round_robin_index= *round_robin_index+1;

}


#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);



    let mut roundtotal = 3;
    let  mut node_id =0 ;
    let  mut node_id2 =0 ;
    let  mut node_idcopy = 0;
    if let Some(node_id_s) = std::env::args().nth(1) {
        node_id = node_id_s.parse()?;
        println!("node_id {}", node_id_s)
    }

    node_idcopy = node_id;
    node_id+=800;
    node_id2= node_id + 10;
    let port:String = node_id.to_string();
    let port2: String = node_id2.to_string();


    let temp_string: String = "127.0.0.2:".to_string();

    let temp_string2: String = "A hello from server ".to_string();
    let together3 = format!("{}{}", temp_string2, node_id);



    let together = format!("{}{}", temp_string, port);
    let together2 = format!("{}{}", temp_string, port2);
    println!("id/port is {}", together);
    println!("id/port together 2 is {}", together2);
    let socket = UdpSocket::bind(together).expect("couldn't connect to address");

    let socket2 = UdpSocket::bind(together2).expect("couldn't connect to address");


    
 
    if node_id == 801 {
        socket.connect("127.0.0.2:812").expect("couldn't connect to address");
        println!("id/port is 12");
    } else if node_id == 802 {
        socket.connect("127.0.0.2:813").expect("couldn't connect to address");
        println!("id/port is 13");
    } else {
        socket.connect("127.0.0.2:811").expect("couldn't connect to address");
        println!("id/port is 111");
    }


    // let socket2 = UdpSocket::bind("127.0.0.2:802").await?;

   


    let mut round_robin_index =1;
    let mut number_of_computed_requests =0;
    let mut number_of_recieved_requests =0;

    let one_seconds
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    = Duration::from_secs(1);
    let seconds = Duration::from_secs(15);
    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(2) {
     
        let remote: Multiaddr = addr.parse()?;
       
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    socket.set_nonblocking(true);
    socket2.set_nonblocking(true);

    let mut warmup = 0;

    let mut sleep_counter=0;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => increment_round_robin( &mut round_robin_index, &mut number_of_computed_requests,&mut number_of_recieved_requests,&mut node_idcopy, &mut roundtotal) ,
            _ => {}
        }


    let k ;


    if(node_idcopy == 1){

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) =>k = n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }

      

        println!("time is {} ", k%30);
   

        if (k%30==0){
           sleep(one_seconds);
            println!("sleeping");
            socket.send(b"F2");
            sleep(seconds);
            println!("woke up");
            node_idcopy = 3;
            roundtotal = 3;
            socket.send(b"R2");
        }
    }




        let mut buf = vec![0; 1024];
        socket.send(together3.as_bytes());
        socket2.recv(&mut buf);
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };


        warmup+=1;


        
        for (i, c) in s.chars().enumerate() {
            // do something with character `c` and index `i`

                if(c == 'E' &&  i == 0){

                    if(node_idcopy == 3)
                    {
                        println!("enterted node 3 turning it to 2");
                        roundtotal = 2;
                        node_idcopy = 2;
                    }

                }else if(c == 'F' && i == 0){

                    if(node_idcopy == 2)
                    {
                        println!("enterted the empty IF");
                        socket.send(b"E2");
                        roundtotal = 2;
                        node_idcopy = 1;
                    }

                    
                }else if(c == 'R' && i == 0){

                    if(node_idcopy == 1)
                    {
                        println!("node woke up 2 saw this");
                        roundtotal = 3;
                        socket.send(b"R1");
                    }

                    if(node_idcopy == 2)
                    {
                        println!("node woke up 3 saw this");
                        roundtotal = 3;
                    }
                }
                break;    
        }
        println!("Received {}", s);

       

        // // socket2.send(b"hello", ).await?;
        // socket2.recv(&mut buf).await?;
        // println!("Received bytes");
      
    }

}