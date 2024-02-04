use worlds_end::*;
use ulid::Ulid;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename="id")]
        uid: String
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

struct GenerateNode {
    id: usize,
}

impl Node<Payload> for GenerateNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            // Single Init at the start of each #[test]
            // Should reply with "init_ok"
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("Serialize response to init")?;
                output.write_all(b"\n").context("Write trailing newline");
                self.id += 1;
            }
            Payload::InitOk => {}
            Payload::Generate => {
                let uid = Ulid::new().to_string();
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::GenerateOk { uid } ,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("Serialize response to echo")?;
                output.write_all(b"\n").context("Write trailing newline");
                self.id += 1;
            }
            Payload::GenerateOk { .. } => {}
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(GenerateNode { id: 0})
}
