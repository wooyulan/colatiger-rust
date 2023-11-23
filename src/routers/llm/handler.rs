use axum::{
    response::sse::{Event, KeepAlive, Sse},
};
use std::{time::Duration, convert::Infallible};
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};


pub  async fn chat_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>>{




    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}