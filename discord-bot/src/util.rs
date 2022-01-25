use serenity::model::Permissions as SerenityPermissions;

use crate::event_handler::polkadot::runtime_types::pallet_discord::primitives::Permissions;

pub fn calculate_permissions(perms: Vec<Permissions>) -> SerenityPermissions {
    let mut permissions: u64 = 0;
    for perm in perms {
        permissions |= u64::pow(2, perm as u32);
    }
    SerenityPermissions::from_bits_truncate(permissions)
}

#[macro_export]
macro_rules! handle_error {
    ($result: expr) => {
        if let Err(why) = $result {
            eprintln!("{:?}", why);
        }
    };
}
