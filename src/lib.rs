#[cfg(feature = "app-states")]
mod application_states;
#[cfg(feature = "app-states")]
pub use application_states::*;

#[cfg(feature = "date")]
pub mod date_time;

#[cfg(feature = "event-loop")]
pub mod events_loop;

#[cfg(feature = "timers")]
mod my_timer;
#[cfg(feature = "timers")]
pub use my_timer::{MyTimer, MyTimerTick};

#[cfg(feature = "objects-pool")]
pub mod objects_pool;

#[cfg(feature = "task-completion")]
mod task_completion;
#[cfg(feature = "task-completion")]
pub use task_completion::{TaskCompletion, TaskCompletionAwaiter, TaskCompletionError};

#[cfg(feature = "base64")]
pub mod base64;
#[cfg(feature = "hex")]
pub mod hex;

#[cfg(feature = "array-bytes-iterator")]
pub mod array_of_bytes_iterator;

//std based libraries

pub mod placeholders;

pub mod vec_maybe_stack;

pub mod file_utils;

pub mod auto_shrink;

mod binary_payload_builder;
pub use binary_payload_builder::*;

pub mod grouped_data;

pub mod lazy;

pub mod duration_utils;

pub mod linq;

mod unsafe_value;
pub use unsafe_value::*;

pub mod remote_endpoint;

mod maybe_short_string;
pub use maybe_short_string::*;

mod sorted_ver_with_2_keys;
pub use sorted_ver_with_2_keys::*;

mod min_value;
pub use min_value::*;

mod max_value;
pub use max_value::*;

mod short_string;
pub mod sorted_vec;
pub use short_string::*;
mod str_or_string;
pub use str_or_string::*;
mod string_builder;
pub use string_builder::StringBuilder;
mod as_slice_or_vec;
pub use as_slice_or_vec::*;
pub mod slice_of_u8_utils;
pub mod str_utils;
