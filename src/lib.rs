use serde::{de::DeserializeOwned, Deserialize, Serialize};
use anyhow::Context;
use std::io::StdoutLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Init {
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body<Payload> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Node<Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}


pub fn main_loop<Payload, S>(mut state: S) -> anyhow::Result<()> where S: Node<Payload>, Payload: DeserializeOwned {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();

    let mut stdout = std::io::stdout().lock();

    for input in inputs {
        let input = input.context("Input from STDIN could not be deserialized")?;
        state.step(input, &mut stdout)?;
    }
    Ok(())
}
