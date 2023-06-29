// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};


fn main() {
    #[cfg(feature = "ssr")]
    {
        use dioxus_fullstack::prelude::*;
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                pre_cache_static_routes_with_props(
                    &ServeConfigBuilder::new_with_router(
                        dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
                    )
                    .incremental(IncrementalRendererConfig::default().static_dir("./docs"))
                    .index_path("./docs/index.html")
                    .build(),
                )
                .await
                .unwrap();
            });
    }

    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        incremental,
    });
}


#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
pub enum Route {
    #[route("/")]
    Homepage {},
    #[route("/hello")]
    Testing {},
}

#[inline_props]
fn Homepage(cx: Scope) -> Element{
    render!{
        "homepage"
        Link {
            target: Route::Testing {}
        }
    }
}


#[inline_props]
fn Testing(cx: Scope) -> Element{
    render!{
        "testing"
        Link {
            target: Route::Homepage {}
        }
    }
}
