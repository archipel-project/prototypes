use bytes::BytesMut;

#[derive(Clone, Debug)]
pub struct PacketFrame {
    /// The ID of the decoded packet.
    pub id: i32,
    /// The contents of the packet after the leading VarInt ID.
    pub body: BytesMut,
}
