use super::*;
use jid::Jid;
use super::Event;
use super::ToEvent;
use std::io;
use xmpp_xml::Element;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="message")]
pub struct Message {
    generic: GenericMessage,
    message_type: String,
    pub body: String,
}

impl Message {
    pub fn new<S: ToString + ?Sized>(to: Jid, msg: &S) -> Message {
        Message {
            generic: GenericMessage::new(to),
            message_type: String::from("chat"),
            body: msg.to_string(),
        }
    }
}

impl ToXmlElement for Message {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = self.generic.to_element().unwrap();

        element
            .append_new_child(("", "body"))
            .set_text(self.body.clone());
        Ok(element)
    }
}

impl FromXmlElement for Message {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> {
        let generic = GenericMessage::from_element(e).unwrap();

        Ok(Message {
               generic,
               message_type: "chat".to_string(),
               body: String::new(),
           })
    }
}
