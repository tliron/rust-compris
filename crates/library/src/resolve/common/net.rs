use super::{
    super::{super::normal::*, context::*, error::*, resolve::*, result::*},
    from_str::*,
};

use {duplicate::*, kutil_std::error::*, std::net::*};

#[duplicate_item(
  _Resolved;
  [IpAddr];
  [Ipv6Addr];
  [Ipv4Addr];
  [SocketAddr];
  [SocketAddrV6];
  [SocketAddrV4];
)]
impl<ContextT, ErrorT> Resolve<_Resolved, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<_Resolved, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        resolve_from_str(self, context, ancestor, errors)
    }
}
