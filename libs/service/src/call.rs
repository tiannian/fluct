use thiserror::Error;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
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
    pub(crate) receiver: UnboundedReceiver<(Request, Option<oneshot::Sender<Response>>)>,
}

impl<Request, Response> Hander<Request, Response> {
    pub async fn recv(
        &mut self,
    ) -> Result<(Request, Option<oneshot::Sender<Response>>), CallError> {
        self.receiver.recv().await.ok_or(CallError::ChannelClosed)
    }
}

pub struct Caller<Request, Response> {
    pub(crate) sender: UnboundedSender<(Request, Option<oneshot::Sender<Response>>)>,
}

impl<Request, Response> Clone for Caller<Request, Response> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Request, Response> Caller<Request, Response> {
    pub fn call(&self, req: Request) -> Result<oneshot::Receiver<Response>, CallError> {
        let (sender, receiver) = oneshot::channel();

        self.sender
            .send((req, Some(sender)))
            .map_err(|_| CallError::ChannelClosed)?;

        Ok(receiver)
    }

    pub fn send(&self, req: Request) -> Result<(), CallError> {
        self.sender
            .send((req, None))
            .map_err(|_| CallError::ChannelClosed)?;

        Ok(())
    }
}

pub fn local_rpc<Request, Response>() -> (Hander<Request, Response>, Caller<Request, Response>) {
    let (sender, receiver) = unbounded_channel();

    (Hander { receiver }, Caller { sender })
}
