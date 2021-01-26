use crate::database::*;
use crate::util::result::Result;

use super::PermissionCalculator;

use num_enum::TryFromPrimitive;
use std::ops;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
pub enum ChannelPermission {
    View = 1,
    SendMessage = 2,
    ManageMessages = 4,
}

bitfield! {
    pub struct ChannelPermissions(MSB0 [u32]);
    u32;
    pub get_view, _: 31;
    pub get_send_message, _: 30;
    pub get_manage_messages, _: 29;
}

impl_op_ex!(+ |a: &ChannelPermission, b: &ChannelPermission| -> u32 { *a as u32 | *b as u32 });
impl_op_ex_commutative!(+ |a: &u32, b: &ChannelPermission| -> u32 { *a | *b as u32 });

/*pub async fn calculate(user: &User, target: &Channel) -> Result<u32> {
    match target {
        Channel::SavedMessages { user: owner, .. } => {
            if &user.id == owner {
                Ok(ChannelPermission::View
                    + ChannelPermission::SendMessage
                    + ChannelPermission::ManageMessages)
            } else {
                Ok(0)
            }
        }
        Channel::DirectMessage { recipients, .. } => {
            if recipients.iter().find(|x| *x == &user.id).is_some() {
                if let Some(recipient) = recipients.iter().find(|x| *x != &user.id) {
                    let perms = super::user::get(&user, recipient).await?;

                    if perms.get_send_message() {
                        return Ok(ChannelPermission::View + ChannelPermission::SendMessage);
                    }

                    return Ok(ChannelPermission::View as u32);
                }
            }

            Ok(0)
        }
        Channel::Group { recipients, .. } => {
            if recipients.iter().find(|x| *x == &user.id).is_some() {
                Ok(ChannelPermission::View + ChannelPermission::SendMessage)
            } else {
                Ok(0)
            }
        }
    }
}

pub async fn get(user: &User, target: &Channel) -> Result<ChannelPermissions<[u32; 1]>> {
    Ok(ChannelPermissions([calculate(&user, &target).await?]))
}*/

impl<'a> PermissionCalculator<'a> {
    pub async fn calculate_channel(self) -> Result<u32> {
        let channel = if let Some(channel) = self.channel {
            channel
        } else {
            unreachable!()
        };

        match channel {
            Channel::SavedMessages { user: owner, .. } => {
                if &self.perspective.id == owner {
                    Ok(ChannelPermission::View
                        + ChannelPermission::SendMessage
                        + ChannelPermission::ManageMessages)
                } else {
                    Ok(0)
                }
            }
            Channel::DirectMessage { recipients, .. } => {
                if recipients.iter().find(|x| *x == &self.perspective.id).is_some() {
                    if let Some(recipient) = recipients.iter().find(|x| *x != &self.perspective.id) {
                        let perms = self.for_user(recipient).await?;
    
                        if perms.get_send_message() {
                            return Ok(ChannelPermission::View + ChannelPermission::SendMessage);
                        }
    
                        return Ok(ChannelPermission::View as u32);
                    }
                }
    
                Ok(0)
            }
            Channel::Group { recipients, .. } => {
                if recipients.iter().find(|x| *x == &self.perspective.id).is_some() {
                    Ok(ChannelPermission::View + ChannelPermission::SendMessage)
                } else {
                    Ok(0)
                }
            }
        }
    }

    pub async fn for_channel(self) -> Result<ChannelPermissions<[u32; 1]>> {
        Ok(ChannelPermissions([ self.calculate_channel().await? ]))
    }
}