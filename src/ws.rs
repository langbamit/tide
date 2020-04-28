
use async_tungstenite::async_std::connect_async;
use async_tungstenite::{accept_async, tungstenite::Error};

use async_std::sync;
use async_std::future::Future;
use async_std::io::BufReader;
use async_std::io::{self, Read, Write};
use crate::utils::BoxFuture;
use crate::{Endpoint, Request, Response, Result};
use crate::http::{mime, Body, StatusCode};
use std::sync::Arc;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use std::pin::Pin;

#[derive(Debug)]
pub struct WsEndpoint<F, Fut, State>
where
    F: Fn(Request<State>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + Sync + 'static,
{
    handler: Arc<F>,
    __state: PhantomData<State>,
    __fut: PhantomData<Fut>,
}





impl<F, Fut, State> Endpoint<State> for WsEndpoint<F, Fut, State>
where
    State: Send + Sync + 'static,
    F: Fn(Request<State>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + Sync + 'static, {
    fn call<'a>(&'a self, mut req: Request<State>) -> BoxFuture<'a, Result> { 
        let handler = self.handler.clone();
        let reader = req.request.take_body();
        Box::pin(async move {
            // let encoder = encode();
            let mut res = Response::new(StatusCode::Ok);
            res.res.insert_header("Cache-Control", "no-cache").unwrap();
            res.res.set_content_type(mime::SSE);

            // let body = Body::from_reader(BufReader::new(encoder), None);
            let body = Body::empty();
            res.set_body(body);

            Ok(res)
        })
    }

}
#[derive(Debug)]
pub(crate) struct WsStream {
    reader: Body,
    // writer:
}

impl Read for WsStream {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

// impl Write for WsStream {
//     fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8])
//             -> Poll<io::Result<usize>> { todo!() }
//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> { todo!() }
//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> { todo!() }
    
// }


// pin_project_lite::pin_project! {
//     /// An SSE protocol encoder.
//     #[derive(Debug)]
//     pub struct Encoder {
//         buf: Option<Vec<u8>>,
//         #[pin]
//         receiver: sync::Receiver<Vec<u8>>,
//         cursor: usize,
//     }
// }

// pub fn encode() -> Encoder {
//     let (sender, receiver) = sync::channel(1);
//     let encoder = Encoder {
//         receiver,
//         buf: None,
//         cursor: 0,
//     };
//     encoder
// }


