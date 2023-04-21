use dvorak_message::message::{Message, MessageType};
use tokio::{net::{TcpListener, TcpStream}, task::JoinHandle};
use tokio::sync::mpsc::Sender;
use crate::{dctor::Dctor, supervisor::SupervisorMessage};

use super::supervisor::ClientSupervisor;

/// representing the server,
/// listening the incoming client and io,
/// start and terminal the whole application
///
/// # example
/// ```
/// let server = Server::new("127.0.0.1:9998");
/// server.listen();
/// ```
pub struct Server {
    tcp_listener: TcpListener,
    supervisor_handler: JoinHandle<()>,
    supervisor_sender: Sender<SupervisorMessage>
}

impl Server {
    /// construct a Server
    pub async fn new(host: &str) -> Self {
        let tcp_listener = TcpListener::bind(host).await.unwrap();
        let (mut client_supervisor, supervisor_sender) = ClientSupervisor::new();

        let supervisor_handler = tokio::spawn(async move{
            client_supervisor.listen();
        });

        Server {
            tcp_listener,
            supervisor_handler,
            supervisor_sender
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let (mut incoming_client, _) = self.tcp_listener.accept().await.unwrap();

            let first = Server::check_login(&mut incoming_client).await;
            if first.is_err() {
                Message::send(
                    &mut incoming_client,
                    Message::new(
                        MessageType::Text("need login".to_string()),
                        "<Server>".to_string(),
                        String::new(),
                    ),
                )
                .await
                .unwrap();
                continue;
            }

            let username = first.unwrap();
            println!("{username} connecting");

            self.client_supervisor
        }
    }

    async fn check_login(tcp_stream: &mut TcpStream) -> Result<String, ()> {
        let message = Message::read_from(tcp_stream).await.unwrap();
        if message.is_none() {
            return Err(());
        }
        let message = message.unwrap();
        if message.message_type != MessageType::Login {
            return Err(());
        }

        Ok(message.username)
    }
}
