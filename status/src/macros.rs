#[macro_export]
macro_rules! impl_default_from_new {
    ($struct: ident) => {
        impl Default for $struct {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_packet_id {
    ($struct: ident) => {
        impl $crate::packet::traits::PacketId for $struct {
            fn get_packet_id(&self) -> usize {
                self.packet_id
            }
        }
    };
}
