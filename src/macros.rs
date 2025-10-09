// declare multiple plugin imports
#[macro_export]
macro_rules! plugins {
    ( $( ($mod_name:ident, $plugin_type:ident) ),+ $(,)? ) => {
        $(
            pub mod $mod_name;
            pub use $mod_name::$plugin_type;
        )+
    };

    ( $( ($mod_name:ident) ),+ $(,)? ) => {
        $(
            pub mod $mod_name;
            pub use $mod_name::*;
        )+
    };
}

// declare multiple mod definitions
#[macro_export]
macro_rules! mods {
    ( $( $mod_name:ident ),+ $(,)? ) => {
        $(
            mod $mod_name;
        )+
    };
}

// expose modules to everyone
#[macro_export]
macro_rules! public {
    ( $( $mod_name:ident ),+ $(,)? ) => {
        $(
            pub mod $mod_name;
            pub use self::$mod_name::*;
        )+
    };
}

// expose modules to the parent module
#[macro_export]
macro_rules! private {
    ( $( $mod_name:ident ),+ $(,)? ) => {
        $(
            mod $mod_name;
            use self::$mod_name::*;
        )+
    };
}

// Import the crates (or a specific crates) prelude
#[macro_export]
macro_rules! prelude {
    () => {
        use crate::prelude::*;
    };
    ($crate_name:ident) => {
        use $crate_name::prelude::*;
    };
}

// Get data from the parent module in a subfolder, i.e. src/modules/world/map -> src/modules/world/map/assets
#[macro_export]
macro_rules! module {
    ( $( $mod_name:ident ),+ $(,)? ) => {
        $(
            use super::super::$mod_name::*;
        )+
    };
}

// Call all game module specific functions
#[macro_export]
macro_rules! game_module_build {
    ($ty:ty) => {
        impl bevy::app::Plugin for $ty {
            fn build(&self, app: &mut bevy::prelude::App) {
                self.assets(app);
                self.configure_loading_state(app);
                self.resources(app);
                self.types(app);
                self.states(app);
                self.system_sets(app);
                self.messages(app);
                self.plugins(app);
                self.systems(app);
            }
        }
    };
}
