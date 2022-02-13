use async_std::{io::prelude::BufReadExt, prelude::*};
use serde::Serialize;
use std::error::Error;

// async_std, serde_json, tokioそれぞれ独自のエラー型を定義しているがFromトレイトからこの型に変換できる
// 実際はanyhowクレートをつかったほうが良い
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

use serde::de::DeserializeOwned;

pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
where
    S: async_std::io::BufRead + Unpin,
    P: DeserializeOwned,
{
    inbound.lines().map(|line_result| -> ChatResult<P> {
        let line = line_result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
