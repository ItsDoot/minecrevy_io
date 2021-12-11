pub use codec::*;
pub use frame::*;
pub use io::*;

mod codec;
mod frame;
mod io;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Play,
}

impl Default for ProtocolState {
    fn default() -> Self {
        Self::Handshake
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct ProtocolVersion {
    protocol: i32,
    names: &'static [&'static str],
}

impl ProtocolVersion {
    pub const V_1_16_4: Self = Self::new(754, &["1.16.4", "1.16.5"]);

    pub const fn all() -> &'static [Self] {
        &[Self::V_1_16_4]
    }

    const fn new(protocol: i32, names: &'static [&'static str]) -> Self {
        Self {
            protocol,
            names,
        }
    }

    pub const fn protocol(&self) -> i32 {
        self.protocol
    }

    pub const fn names(&self) -> &'static [&'static str] {
        self.names
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
