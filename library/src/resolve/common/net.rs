use crate::impl_resolve_from_str;

use {duplicate::*, std::net::*};

#[duplicate_item(
  ResolvedT;
  [IpAddr];
  [Ipv6Addr];
  [Ipv4Addr];
  [SocketAddr];
  [SocketAddrV6];
  [SocketAddrV4];
)]
impl_resolve_from_str!(ResolvedT);
