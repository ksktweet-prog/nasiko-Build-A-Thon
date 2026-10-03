use anyhow::Result;

use crate::{
    decode_calls,
    ToolCall,
    ToolDef,
};

#[derive(Debug)]
enum State {
    Searching,
    Reading,
}

pub struct StreamDecoder {

    state: State,

    buffer: String,
}

impl StreamDecoder {

    pub fn new() -> Self {

        Self {
            state: State::Searching,
            buffer: String::new(),
        }
    }

    pub fn push_chunk(
        &mut self,
        chunk: &str,
        tools: &[ToolDef],
    ) -> Result<Vec<ToolCall>>
    {
        self.buffer.push_str(chunk);

        match self.state {

            State::Searching => {

                if self.buffer.contains("<<") {
                    self.state =
                        State::Reading;
                }
            }

            State::Reading => {}
        }

        if !self.buffer.contains(">>") {
            return Ok(vec![]);
        }

        let calls =
            decode_calls(
                &self.buffer,
                tools,
            )?;

        self.buffer.clear();

        self.state =
            State::Searching;

        Ok(calls)
    }
}