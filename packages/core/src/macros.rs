#[macro_export]
macro_rules! partial {
    ($( #[$attr:meta] )* pub struct $name:ident {
        $( pub $field_name:ident: $field_type:ty, )*
    }) => {
        $( #[$attr] )*
        pub struct $name {
            $(pub $field_name: Option<$field_type>,)*
        }

        impl $name {
            pub fn merge(&mut self, other: $name) {
                $( if other.$field_name.is_some() { self.$field_name = other.$field_name; } )*
            }
        }
    }
}
