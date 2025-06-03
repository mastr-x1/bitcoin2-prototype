use libp2p::{
    identity, noise, ping,
    swarm::{Swarm, SwarmEvent},
    tcp, Multiaddr, PeerId, Transport,
};
use std::error::Error;
use tokio::sync::mpsc;
use crate::blockchain::Blockchain;

pub struct P2PNetwork {
    pub swarm: Swarm<ping::Behaviour>,
    pub blockchain: Blockchain,
    pub command_sender: mpsc::Sender<NetworkCommand>,
    pub event_receiver: mpsc::Receiver<NetworkEvent>,
}

pub enum NetworkCommand {
    BroadcastBlock(Block),
    BroadcastTransaction(Transaction),
}

pub enum NetworkEvent {
    BlockReceived(Block),
    TransactionReceived(Transaction),
}

impl P2PNetwork {
    pub async fn new(blockchain: Blockchain) -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&local_key)?)
            .multiplex(libp2p::yamux::YamuxConfig::default())
            .boxed();
        
        let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
        
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        let (command_sender, command_receiver) = mpsc::channel(32);
        let (event_sender, event_receiver) = mpsc::channel(32);
        
        tokio::spawn(Self::network_event_loop(
            swarm,
            blockchain.clone(),
            command_receiver,
            event_sender,
        ));
        
        Ok(Self {
            swarm,
            blockchain,
            command_sender,
            event_receiver,
        })
    }
    
    async fn network_event_loop(
        mut swarm: Swarm<ping::Behaviour>,
        mut blockchain: Blockchain,
        mut command_receiver: mpsc::Receiver<NetworkCommand>,
        event_sender: mpsc::Sender<NetworkEvent>,
    ) {
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on: {}", address);
                        }
                        SwarmEvent::Behaviour(ping::Event { peer, result }) => {
                            match result {
                                Ok(rtt) => println!("Ping from {}: {}ms", peer, rtt.as_millis()),
                                Err(e) => println!("Ping error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
                command = command_receiver.recv() => {
                    if let Some(cmd) = command {
                        match cmd {
                            NetworkCommand::BroadcastBlock(block) => {
                                // Broadcast block to peers
                                blockchain.add_block(block);
                            }
                            NetworkCommand::BroadcastTransaction(tx) => {
                                blockchain.add_transaction(tx);
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub async fn connect(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.dial(addr)?;
        Ok(())
    }
    
    pub async fn broadcast_block(&self, block: Block) {
        let _ = self.command_sender.send(NetworkCommand::BroadcastBlock(block)).await;
    }
    
    pub async fn broadcast_transaction(&self, tx: Transaction) {
        let _ = self.command_sender.send(NetworkCommand::BroadcastTransaction(tx)).await;
    }
}