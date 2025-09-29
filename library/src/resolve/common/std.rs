use super::impl_resolve_from_str;

use {
    duplicate::*,
    std::{net::*, path::*},
};

#[duplicate_item(
  ResolvedT;
  [IpAddr];
  [Ipv6Addr];
  [Ipv4Addr];
  [SocketAddr];
  [SocketAddrV6];
  [SocketAddrV4];
  [PathBuf];
)]
impl_resolve_from_str!(ResolvedT);
