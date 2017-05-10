use std::str::FromStr;
use xmpp_proto::Jid;
use xmpp_proto::events::GenericMessage;

#[test]
fn create_a_generic_message() {
    let mut g = GenericMessage::new("");

    let _ = g.set_id(Some("ok"));

    // GenericMessage can have an ID
    match g.get_id() {
        Some(id) => assert_eq!("ok", id),
        None => {}
    }

    let _ = g.set_to("test@example.com");

    // GenericMessage should have an to
    assert_eq!(&Jid::from_str("test@example.com").unwrap(), g.get_to());
    assert_eq!("test@example.com", g.get_to().to_string());

    let _ = g.set_from(Some("test@example.com"));
    // GenericMessage should have an from not sent by the client but by the server to the end
    // client
    match g.get_from() {
        Some(from) => {
            assert_eq!(&Jid::from_str("test@example.com").unwrap(), from);
            assert_eq!("test@example.com", from.to_string())
        }
        None => {}
    }
}
