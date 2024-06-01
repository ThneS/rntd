//! event frame module
//! Frame (T:u64 0x5a5a5a5a5a5a5a5a, L: len of V, V: Event(JsonString))
//! enum with simple event(task_id, alarm_id, msg:JsonString) and heart beat event
//! use tokio_util codec to encode and decode
//! use sqlx to operate database
//! use tracing to log
//! serde to serialize and deserialize
use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    SimpleEvent {
        task_id: u64,
        alarm_id: u64,
        msg: String,
    },
    HeartBeat,
}

#[derive(Debug)]
pub struct Frame {
    t: u64,
    l: u64,
    v: String, // JsonString(Event)
}

pub struct FrameCodec;

impl Encoder<Frame> for FrameCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u64(item.t);
        dst.put_u64(item.l);
        dst.put(item.v.as_bytes());
        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 16 {
            return Ok(None);
        }
        let t = src.get_u64();
        let l = src.get_u64();
        if src.len() < l as usize {
            return Ok(None);
        }
        let v = src.split_to(l as usize);
        let v = String::from_utf8(v.to_vec()).unwrap();
        Ok(Some(Frame { t, l, v }))
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use super::*;

    #[test]
    fn test_frame_encode() {
        let event_json = r#"{"task_id":1,"alarm_id":2,"msg":"hello"}"#;
        let frame = Frame {
            t: 0x5a5a5a5a5a5a5a5a,
            l: event_json.len() as u64,
            v: event_json.to_string(),
        };
        println!("{:?}", frame);
        let mut buf = BytesMut::new();
        let mut codec = FrameCodec {};
        codec.encode(frame, &mut buf).unwrap();
        assert_eq!(buf.len(), 16 + event_json.len());
        let t = buf.get_u64();
        let v = buf.get_u64();
        assert!(t == 0x5a5a5a5a5a5a5a5a);
        assert!(v == event_json.len() as u64);
    }

    #[test]
    fn test_frame_decode() {
        let mut buf = BytesMut::from(&[0x5a; 8][..]);
        buf.put_u64(0);
        let mut codec = FrameCodec {};
        let frame = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(frame.t, 0x5a5a5a5a5a5a5a5a);
        assert_eq!(frame.l, 0);
        assert_eq!(frame.v, "".to_string());
    }
    #[test]
    fn event_encode() {
        let event = Event::SimpleEvent {
            task_id: 1,
            alarm_id: 2,
            msg: "hello".to_string(),
        };
        let event_json = r#"{"task_id":1,"alarm_id":2,"msg":"hello"}"#;
        let event_json_encode = serde_json::to_string(&event).unwrap();
        assert!(event_json_encode == *event_json);
    }

    #[test]
    fn event_decode() {
        let event_json = r#"{"task_id":1,"alarm_id":2,"msg":"hello"}"#;
        let event_json_decode = serde_json::from_str(event_json).unwrap();
        match event_json_decode {
            Event::SimpleEvent {
                task_id,
                alarm_id,
                msg,
            } => {
                assert!(task_id == 1);
                assert!(alarm_id == 2);
                assert!(msg == *"hello");
            }
            _ => {
                info!("event_json_decode is not SimpleEvent");
            }
        }
    }
}
