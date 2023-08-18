pub mod database;
pub mod error;
pub mod trade;
pub mod trade_computer;
pub mod transaction;

#[allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[route("/crypto/transactions")]
        CryptosTransactions {},
        #[route("/crypto/trades")]
        CryptosTrades {},
        #[route("/stocks/transactions")]
        StocksTransactions {},
        #[route("/stocks/trades")]
        StocksTrades {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            display: "grid",
            // style: "display: grid; grid-template-columns: repeat(var(--items), 1fr); position: relative; --items: 4;",
            ul {
                margin: 0,
                padding: 0,
                style: "list-style-type: none",
                li { margin: 10, padding: 10, Link { to: Route::CryptosTransactions {  }, "Crypto Transactions" } }
                li { margin: 10, padding: 10, Link { to: Route::CryptosTrades {  }, "Crypto Trades" } }
                li { margin: 10, padding: 10, Link { to: Route::StocksTransactions {  }, "Stocks Transactions" } }
                li { margin: 10, padding: 10, Link { to: Route::StocksTrades {  }, "Stocks Trades" } }
            }
        }
        Outlet::<Route> {}
    }
}

fn CryptosTransactions(cx: Scope) -> Element {
    render! {
        h1 { "Cryptos Transactions : " }
    }
}

fn CryptosTrades(cx: Scope) -> Element {
    render! {
        h1 { "Cryptos Trades : " }
    }
}

fn StocksTransactions(cx: Scope) -> Element {
    render! {
        h1 { "Stocks Transactions : " }
    }
}

fn StocksTrades(cx: Scope) -> Element {
    render! {
        h1 { "Stocks Trades : " }
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}