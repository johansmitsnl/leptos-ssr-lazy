use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    lazy_route, Lazy, LazyRoute, StaticSegment,
};

mod start {
    pub enum Initial {
        Zero,
        Ten,
    }

    pub async fn load_site() -> bool {
        true
    }

    pub async fn login_check() -> bool {
        true
    }
}

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
              path={StaticSegment("")}
              view={move || {
                view! {
                  <Await future={async move { start::load_site().await }} let:_data>
                    <Outlet />
                  </Await>
                }
              }}
            >
              <ParentRoute
                path={StaticSegment("/console")}
                view={Lazy::<ConsoleRoute>::new()}
              >
                <Route
                  path={StaticSegment("/")}
                  view={move || view! { <Redirect path="/console/home-a" /> }}
                />
                <Route path={StaticSegment("/home-a")} view={Lazy::<HomeRouteA>::new()} />
              </ParentRoute>
              <Route path={StaticSegment("/home-b")} view={Lazy::<HomeRouteB>::new()} />
            </ParentRoute>
          </Routes>
        </main>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(initial: start::Initial) -> impl IntoView {
    // Creates a reactive value to update the button
    let initial = match initial {
        start::Initial::Zero => 0i32,
        start::Initial::Ten => 10i32,
    };
    let count = RwSignal::new(initial);
    let on_click = move |_| *count.write() += 1;

    view! {
      <h1>"Welcome to Leptos!"</h1>
      <button on:click={on_click}>"Click Me: " {count}</button>
    }
}

struct HomeRouteA;
#[lazy_route]
impl LazyRoute for HomeRouteA {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <HomePage initial={start::Initial::Zero} /> }.into_any()
    }
}

struct HomeRouteB;
#[lazy_route]
impl LazyRoute for HomeRouteB {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <HomePage initial={start::Initial::Ten} /> }.into_any()
    }
}

struct ConsoleRoute;
#[lazy_route]
impl LazyRoute for ConsoleRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let session = RwSignal::new(true);
        let session_check = Resource::new(
            move || session.try_get().unwrap_or_default(),
            |_| async move { start::login_check().await },
        );

        view! {
          <Transition fallback={move || {
            view! { "" }
          }}>
            {move || match session_check.try_get() {
              Some(Some(true)) => view! { <Outlet /> }.into_any(),
              _ => view! { "Invalid session" }.into_any(),
            }}

          </Transition>
        }
        .into_any()
    }
}
