#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod primitives;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::primitives::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_std::vec;
    use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type DiscordId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
    }

    pub type GuildMemberOf<T> = GuildMember<<T as frame_system::Config>::AccountId>;

    /// A ban from the discord server represented as (member, ban reason).
    pub type Ban<T> = (GuildMemberOf<T>, Vec<u8>);

    pub type RoleOf<T> = Role<<T as pallet::Config>::DiscordId>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_member)]
    pub type GuildMembers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, GuildMemberOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_banned_member)]
    pub type BannedMembers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Ban<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_role)]
    pub type Roles<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, RoleOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_bot)]
    pub type Bots<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BotAdded(T::AccountId),
        MemberAdded(T::AccountId),
        MemberBanned(T::AccountId, Vec<u8>),
        /// Name, color, hoist, position, permissions, mentionable
        RoleCreated(Vec<u8>, u64, bool, u64, Vec<Permissions>, bool),
        RoleAssigned(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Only the bot account has permission to run administrative functions.
        NoPermission,

        NotAMemberOfTheGuild,

        RoleDoesntExist,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1000)]
        pub fn register_bot(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            Bots::<T>::insert(account.clone(), ());

            Self::deposit_event(Event::BotAdded(account));

            Ok(())
        }

        #[pallet::weight(1000)]
        pub fn add_member(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
            ensure!(
                Bots::<T>::get(ensure_signed(origin)?).is_some(),
                Error::<T>::NoPermission
            );

            GuildMembers::<T>::insert(
                member.clone(),
                GuildMember {
                    account: member.clone(),
                    roles: vec![],
                    deaf: false,
                    mute: false,
                },
            );

            Self::deposit_event(Event::MemberAdded(member));

            Ok(())
        }

        #[pallet::weight(1000)]
        pub fn ban_member(
            origin: OriginFor<T>,
            member: T::AccountId,
            reason: Vec<u8>,
        ) -> DispatchResult {
            GuildMembers::<T>::try_mutate(member.clone(), |guild_member| -> DispatchResult {
                ensure!(
                    Bots::<T>::get(ensure_signed(origin)?).is_some(),
                    Error::<T>::NoPermission
                );

                let guild_member = guild_member
                    .take()
                    .ok_or(Error::<T>::NotAMemberOfTheGuild)?;

                BannedMembers::<T>::insert(member.clone(), (guild_member, reason.clone()));

                Self::deposit_event(Event::MemberBanned(member, reason));

                Ok(())
            })
        }

        #[pallet::weight(1000)]
        pub fn create_role(
            origin: OriginFor<T>,
            name: Vec<u8>,
            color: u64,
            hoist: bool,
            position: u64,
            permissions: Vec<Permissions>,
            mentionable: bool,
        ) -> DispatchResult {
            ensure!(
                Bots::<T>::get(ensure_signed(origin)?).is_some(),
                Error::<T>::NoPermission
            );

            Roles::<T>::insert(
                name.clone(),
                Role {
                    id: None,
                    name: name.clone(),
                    color,
                    hoist,
                    position,
                    permissions: permissions.clone(),
                    managed: false,
                    mentionable,
                },
            );

            Self::deposit_event(Event::RoleCreated(
                name,
                color,
                hoist,
                position,
                permissions,
                mentionable,
            ));

            Ok(())
        }

        #[pallet::weight(1000)]
        pub fn assign_role(
            origin: OriginFor<T>,
            target: T::AccountId,
            role_name: Vec<u8>,
        ) -> DispatchResult {
            GuildMembers::<T>::try_mutate(target.clone(), |guild_member| -> DispatchResult {
                ensure!(
                    Bots::<T>::get(ensure_signed(origin)?).is_some(),
                    Error::<T>::NoPermission
                );

                ensure!(
                    Roles::<T>::get(role_name.clone()).is_some(),
                    Error::<T>::RoleDoesntExist
                );

                let mut old_member = guild_member
                    .take()
                    .ok_or(Error::<T>::NotAMemberOfTheGuild)?;
                old_member.roles.push(role_name.clone());

                *guild_member = Some(old_member);

                Self::deposit_event(Event::RoleAssigned(target, role_name));

                Ok(())
            })
        }
    }
}
