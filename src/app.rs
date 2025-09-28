use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    lazy_route, Lazy, LazyRoute, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <AutoReload options={options.clone()} />
          <HydrationScripts options />
          <MetaTags />
        </head>
        <body>
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/leptos-ssr-lazy.css" />

      // sets the document title
      <Title text="Welcome to Leptos" />

      // content for this welcome page
      <Router>
        <main>
          <Routes fallback={|| "Page not found.".into_view()}>
            <ParentRoute
              path={StaticSegment("/console")}
              view={Lazy::<ConsoleRoute>::new()}
            >
              <Route
                path={StaticSegment("/")}
                view={move || view! { <Redirect path="/console/home" /> }}
              />
              <Route path={StaticSegment("/home")} view={Lazy::<HomeRoute>::new()} />
            </ParentRoute>
          </Routes>
        </main>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
      <h1>"Welcome to Leptos!"</h1>
      <button on:click={on_click}>"Click Me: " {count}</button>
    }
}

struct HomeRoute;
#[lazy_route]
impl LazyRoute for HomeRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <HomePage /> }.into_any()
    }
}

struct ConsoleRoute;
#[lazy_route]
impl LazyRoute for ConsoleRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <Outlet /> }.into_any()
    }
}
