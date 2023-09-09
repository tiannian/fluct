use thiserror::Error;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot,
};

pub trait CallableService {
    type Request;

    type Response;

    fn handle(&mut self, req: Self::Request) -> Self::Response;
}

#[derive(Debug, Error)]
pub enum CallError {
    #[error("Channel closed")]
    ChannelClosed,
}

pub struct Hander<Request, Response> {
    receiver: UnboundedReceiver<(Request, oneshot::Sender<Response>)>,
}

impl<Request, Response> Hander<Request, Response> {
    pub async fn recv(&mut self) -> Result<(Request, oneshot::Sender<Response>), CallError> {
        self.receiver.recv().await.ok_or(CallError::ChannelClosed)
    }
}

#[derive(Clone)]
pub struct Caller<Request, Response> {
    sender: UnboundedSender<(Request, oneshot::Sender<Response>)>,
}

impl<Request, Response> Caller<Request, Response> {
    pub fn call(&self, req: Request) -> Result<oneshot::Receiver<Response>, CallError> {
        let (sender, receiver) = oneshot::channel();

        self.sender
            .send((req, sender))
            .map_err(|_| CallError::ChannelClosed)?;

        Ok(receiver)
    }
}
