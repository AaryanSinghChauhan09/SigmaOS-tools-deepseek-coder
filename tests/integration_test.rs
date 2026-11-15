use sigmaos_tools_deepseek_coder::{CoderSession, Message, MessageRole};

#[test]
fn coder_session_integration_aaa() {
    let mut s = CoderSession::new("My Session");
    s.add_message(Message { id: "m1".into(), role: MessageRole::User, content: "Hello".into() }).unwrap();
    assert_eq!(s.message_count(), 1);
}
