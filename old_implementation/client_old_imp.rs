use async_std::prelude::FutureExt;
use futures::prelude::*;
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use libp2p::{identity, Multiaddr, PeerId, ping};
use std::error::Error;
use std::thread;
use tokio::runtime::Runtime;
use tokio::time::*;
fn increment( amount_of_messages: &mut i32){

    *amount_of_messages +=1;

}

async fn client( client_index :i32 )-> Result<(), Box<dyn Error+ Send + Sync>> {
    println!("inside client function: {} !", client_index);
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    //println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    

    for i in 1..4 {
        
        if let Some(addr) = std::env::args().nth(i) {
            let remote: Multiaddr = addr.parse()?;
            swarm.dial(remote)?;
            //println!("Dialed {}", addr)
        }

    }
        
    //}
    let mut amount_of_messages =1;
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => (),
            SwarmEvent::Behaviour(event) => increment(&mut amount_of_messages),
            _ => {}
        }
        println!("amount of messages {}", amount_of_messages);
    }



}






#[async_std::main]
async fn main() {

    let mut rt = Runtime::new().unwrap();



    rt.block_on(async move {

        //client(0).await;
        let mut vec = Vec::new();
        //bonus, you could spawn tasks too

        for i in 1..501 {
            let x=tokio::spawn(async move { client(i).await });
            vec.push(x);
        }

        loop{}

  
    });




}