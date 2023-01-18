use std::error::Error;
use tokio::runtime::Runtime;
use tokio::time::*;
use std::net::UdpSocket;
use std::str;



async fn client( client_index :i32, agent_idx: i32 )-> Result<(), Box<dyn Error+ Send + Sync>> {
    // println!("inside client function: {} !", client_index);
    let local_ip: String = "127.0.0.2:".to_string();
    let local_ip2: String = "127.0.0.2:".to_string();


    let  mut node_id_send= 1000+(client_index*100) + agent_idx*5;
    let  mut node_id_rsv= 1000+(client_index*100) + agent_idx*5+1;

    let port_send: String = node_id_send.to_string();
    let port_rsv: String = node_id_rsv.to_string();
    
    let together_client = format!("{}{}", local_ip, port_send);
    let together_client2 = format!("{}{}", local_ip2, port_rsv);
    let together_client_rsv = format!("{}{}", local_ip, port_rsv);
    // println!{"my port that im supposed to recieve on is {} ", together_client2};
    let socket_send = UdpSocket::bind(together_client).expect("couldn't connect to address");
    let socket_rsv = UdpSocket::bind(together_client_rsv).expect("couldn't connect to address");
 
        
    let mut amount_of_messages_sent =0;
    let mut amount_of_messages_rsv =0;


    let seconds = Duration::from_secs(1);
    socket_rsv.set_read_timeout(Some(seconds)).expect("set_read_timeout call failed");
    socket_send.set_nonblocking(true);

    loop {
        

        let mut buf = vec![0; 1024];
        socket_rsv.recv(&mut buf);
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        
          
    for (i, c) in s.chars().enumerate() {
        // do something with character `c` and index `i`
            if(c == 'H' &&  i == 0){
                println!("{}", s);
                amount_of_messages_rsv+=1;
                println!("clnt {} rsv {} ",client_index, amount_of_messages_rsv);
            }
            break;
    }

        let mut node_send_agent = 901 + agent_idx*10;
        let temp_send_agent =format!("{}{}", local_ip, node_send_agent.to_string()); 
        socket_send.connect(temp_send_agent).expect("couldn't connect to address");
        socket_send.send(together_client2.to_owned().as_bytes());

        amount_of_messages_sent+=1;
        println!("clnt {} snt {} ", client_index,amount_of_messages_sent);
        let one_seconds= Duration::from_millis(100);
        sleep(one_seconds).await;
       
    }



}




async fn agent(agent_idx: i32 )-> Result<(), Box<dyn Error+ Send + Sync>> {
    println!("inside agent function! {} ", agent_idx);
    let local_ip: String = "127.0.0.2:".to_string();

    let mut one_dropped=false;
    let mut two_dropped=false;
    let mut three_dropped=false;
    let mut amountofmessages = 0;
    let  node_id_send= 900 + agent_idx*10;
    let  node_id_rsv= 901 + agent_idx*10;
    let  node_id_rsv_server= 902 + agent_idx*10;


    

    let port_send: String = node_id_send.to_string();
    let port_rsv: String = node_id_rsv.to_string();
    let port_rsv_server: String = node_id_rsv_server.to_string();

    let together_client = format!("{}{}", local_ip, port_send);
    let together_client_rsv = format!("{}{}", local_ip, port_rsv);
    let together_server_rsv = format!("{}{}", local_ip, port_rsv_server);


    let socket_send = UdpSocket::bind(together_client).expect("couldn't connect to address");
    let socket_rsv = UdpSocket::bind(together_client_rsv).expect("couldn't connect to address");
    let socket_rsv_server = UdpSocket::bind(together_server_rsv).expect("couldn't connect to address");
 

    let seconds = Duration::from_millis(10);
    let milli_seconds = Duration::from_millis(100);
    socket_rsv.set_read_timeout(Some(seconds)).expect("set_read_timeout call failed");
    socket_rsv_server.set_read_timeout(Some(seconds)).expect("set_read_timeout call failed");
    socket_send.set_nonblocking(true);

    let mut round_robin_counter=1;
    loop{
        let mut client_buf = vec![0; 1024];
        socket_rsv.recv(&mut client_buf);
        let client_string = match str::from_utf8(&client_buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        
        let mut server_buf = vec![0; 1024];
        socket_rsv_server.recv(&mut server_buf);
        let server_string = match str::from_utf8(&server_buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };



        for (i, c) in server_string.chars().enumerate() {
            // do something with character `c` and index `i`
            if(c == '1' )&& i==0{
                 one_dropped=true;
      
            }else if (c == '2' )&& i==0{
                
                 two_dropped=true;
                
            }else if (c == '3' )&& i==0{
     
                 three_dropped=true;
            }else if(c == '4' )&& i==0{
                one_dropped=false;
     
           }else if (c == '5' )&& i==0{
               
                two_dropped=false;
               
           }else if (c == '6' )&& i==0{
    
                three_dropped=false;
           }

            break;
        }

        
          
        for (i, c) in client_string.chars().enumerate() {
            //println!("client string is {}",client_string);
            // do something with character `c` and index `i`
            if(c == '1' ||  c == '2'||  c == '3'||  c == '4'||  c == '5'||  c == '6'||  c == '7'||  c == '8'||  c == '9' )&& i==0{
                
                if round_robin_counter==1{
                    if one_dropped {
                        round_robin_counter+=1;
                    }
                    else{
                        socket_send.connect("127.0.0.2:826").expect("couldn't connect to address");
                        socket_send.send(client_string.as_bytes());
                        amountofmessages+=1;
                    }
                }
                 if round_robin_counter==2{
                    if two_dropped {
                        round_robin_counter+=1;
                    }
                    else{
                        socket_send.connect("127.0.0.2:827").expect("couldn't connect to address");
                        socket_send.send(client_string.as_bytes());
                        amountofmessages+=1;
                    }

                }
                if round_robin_counter==3 {
                    if three_dropped {
                        round_robin_counter=1;
                        socket_send.connect("127.0.0.2:826").expect("couldn't connect to address");
                        socket_send.send(client_string.as_bytes());
                    }else{
                        round_robin_counter=0;
                        socket_send.connect("127.0.0.2:828").expect("couldn't connect to address");
                        socket_send.send(client_string.as_bytes());
                    }
                    amountofmessages+=1;

                    
                }
            
                round_robin_counter+=1;
            }
            println!("agnt msgs {}", amountofmessages);
            break;
        }


    }
  


}





#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut rt = Runtime::new().unwrap();
    let  mut node_id =0 ;
    if let Some(node_id_s) = std::env::args().nth(1) {
        node_id = node_id_s.parse()?;
        println!("node_id {}", node_id_s);
    }
    rt.block_on(async move {

        //client(0).await;
        let mut vec = Vec::new();
        //bonus, you could spawn tasks too
        let x=tokio::spawn(async move { agent(node_id).await });
        vec.push(x);
        for i in 1..501{
            let x=tokio::spawn(async move { client(i,node_id).await });
            vec.push(x);
        }

        loop{}

  
    })




}