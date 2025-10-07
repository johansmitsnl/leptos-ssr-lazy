use leptos::{
    ev::KeyboardEvent,
    logging::{debug_warn, log},
    prelude::*,
};
use leptos_fluent::{leptos_fluent, move_tr};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    lazy_route, path, Lazy, LazyRoute, StaticSegment,
};
use leptos_use::{use_event_listener, whenever};

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

    leptos_fluent! {
        // Path to the locales directory, relative to Cargo.toml.
        locales: "./locales",
        // Initial language when the user don't load any with
        // the provided configuration.
        default_language: "en",
        // Check translations correctness in the specified files.
        // #[cfg(debug_assertions)]
        // check_translations: "./src/**/*.rs",

        // Client side options
        // -------------------
        // Synchronize `<html lang="...">` attribute with
        // current active language.
        sync_html_tag_lang: true,
        // Synchronize `<html dir="...">` attribute with `"ltr"`,
        // `"rtl"` or `"auto"` depending on active language.
        sync_html_tag_dir: true,
        // Update language on local storage when changes.
        set_language_to_local_storage: true,
        // Get initial language from `navigator.languages`
        // if not found in local storage.
        initial_language_from_navigator: true,
        // Set initial language of user from navigator to a cookie.
        initial_language_from_navigator_to_cookie: true,
        // Attributes to set for language cookie.
        // By default `""`.
        cookie_attrs: "Secure; Path=/; Max-Age=600",
        // Update language on cookie when the language changes.
        set_language_to_cookie: true,

        // Server side options
        // -------------------
        // Set initial language from the `Accept-Language`
        // header of the request.
        initial_language_from_accept_language_header: true,

        // Server and client side options
        // ------------------------------
        // Name of the cookie to get and set the current active
        // language. By default `"lf-lang"`.
        cookie_name: "lang",
        // Set initial language from cookie.
        initial_language_from_cookie: true,
    };

    let site_load = OnceResource::new_blocking(async move {
        debug_warn!("App | Loading site from the api");
        start::load_site().await
    });
    provide_context(site_load);

    view! {
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/leptos-ssr-lazy.css" />

      // sets the document title
      <Title text="Welcome to Leptos" />

      // content for this welcome page
      <Router>
        <main>
          <Routes fallback={NotFound}>
            <ParentRoute
              path={StaticSegment("")}
              view={move || {
                view! {
                  <Await future={async move { site_load.await }} let:_data>
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
              <Route path={StaticSegment("")} view={Lazy::<LazyRoot>::new()} />
              <Route path={path!(":page_id")} view={Lazy::<LazyRoot>::new()} />
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
    let version_check = OnceResource::new(async move {
        debug_warn!("App | Loading site from the api number: {}", count.get());
        String::from("Version OK")
    });

    let _ = whenever(
        move || count.try_with(|state| state == &1) == Some(true),
        move |_, _, _| {
            log!("Do version check");
            version_check.get()
        },
    );

    view! {
      <h1>"Welcome to Leptos!"</h1>
      <button on:click={on_click}>"Click Me: " {count}</button>
      {move || {
        (count.get() == 1)
          .then(|| {
            let cleanup = use_event_listener(
              document().body(),
              leptos::ev::keydown,
              move |evt: KeyboardEvent| {
                debug_warn!("Modal keypress of {} ", &evt.key());
                if Some(evt.key()) == Some(String::from("a")) {
                  debug_warn!("Modal key pressed, performing action closing modal");
                }
              },
            );
            on_cleanup(cleanup);
          })
      }}
    }
}

#[component]
pub(crate) fn NotFound() -> impl IntoView {
    view! {
      <div class="hero bg-base-200 min-h-screen">
        <div class="hero-content flex-col lg:flex-row">
          <div>
            <h1 class="text-5xl font-bold">{move_tr!("page-404")}</h1>
            <p class="py-6">{move_tr!("page-404_description")}</p>
            <a class="btn btn-primary" href="/">
              {move_tr!("page-404_button")}
            </a>
          </div>
        </div>
      </div>
    }
}

#[component]
pub(crate) fn Root() -> impl IntoView {
    view! { "Root page" }
}

struct LazyRoot;
#[lazy_route]
impl LazyRoute for LazyRoot {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <Root /> }.into_any()
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
