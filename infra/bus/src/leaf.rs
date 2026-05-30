/// Defines the standard stamped pub/sub topic leaf exposed by owner-local API crates.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! pubsub_leaf {
    ($module:ident, $topic:ident, $payload:ident) => {
        pub mod $module {
            use super::*;
            use phoxal_infra_bus::pubsub::Stamped;
            use phoxal_infra_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

            pub const TOPIC: &str = $topic;

            pub fn topic(bus: &phoxal_infra_bus::Bus) -> String {
                bus.topic(TOPIC)
            }

            pub fn publisher(
                bus: &phoxal_infra_bus::Bus,
            ) -> phoxal_infra_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<$payload>>>
            {
                phoxal_infra_bus::pubsub::publisher_builder(bus, TOPIC)
            }

            pub fn subscriber_builder(
                bus: &phoxal_infra_bus::Bus,
            ) -> TypedSubscriberBuilder<'_, 'static, Stamped<$payload>> {
                phoxal_infra_bus::pubsub::subscriber_builder(bus, TOPIC)
            }
        }
    };
}

/// Defines the standard request/queryable topic leaf exposed by owner-local API crates.
#[macro_export]
macro_rules! query_leaf {
    ($module:ident, $topic:ident, $request:ty, $response:ty) => {
        pub mod $module {
            use super::*;

            pub const TOPIC: &str = $topic;

            pub fn topic(bus: &phoxal_infra_bus::Bus) -> String {
                bus.topic(TOPIC)
            }

            pub fn get_builder<'a>(
                bus: &'a phoxal_infra_bus::Bus,
                request: &'a $request,
            ) -> phoxal_infra_bus::zenoh_typed::TypedGetBuilder<'a, 'static, $response> {
                phoxal_infra_bus::query::get_builder(bus, TOPIC, request)
            }

            pub fn queryable_builder(
                bus: &phoxal_infra_bus::Bus,
            ) -> phoxal_infra_bus::Result<
                phoxal_infra_bus::zenoh_typed::TypedQueryableBuilder<
                    '_,
                    'static,
                    $request,
                    $response,
                >,
            > {
                phoxal_infra_bus::query::queryable_builder(bus, TOPIC)
            }
        }
    };
}

/// Defines the transparent map-tile request schema wrapper used by query API leaves.
#[macro_export]
macro_rules! request_schema {
    ($name:ident, $schema:literal) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub MapTileRequest);

        impl TypedSchema for $name {
            const SCHEMA_NAME: &'static str = $schema;
            const SCHEMA_VERSION: u32 = 1;
        }
    };
}
