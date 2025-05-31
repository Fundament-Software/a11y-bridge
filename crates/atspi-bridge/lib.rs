// SPDX-FileCopyrightText: 2025 2025 Fundament Software SPC <https://fundament.software>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result as AResult;
use atspi::events::window::ActivateEvent;
use atspi::{AccessibilityConnection, State, events::WindowEvents};
use atspi_connection::set_session_accessibility;
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	proxy_ext::ProxyExt,
	text::TextProxy,
};
use std::error::Error;
use tokio_stream::StreamExt;
use zbus::{Connection, proxy::CacheProperties};

const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
const REGISTRY_PATH: &str = "/org/a11y/atspi/accessible/root";
const ACCCESSIBLE_INTERFACE: &str = "org.a11y.atspi.Accessible";

async fn get_registry_accessible(conn: &Connection) -> AResult<AccessibleProxy<'static>> {
	let registry = AccessibleProxy::builder(conn)
		.destination(REGISTRY_DEST)?
		.path(REGISTRY_PATH)?
		.interface(ACCCESSIBLE_INTERFACE)?
		.cache_properties(CacheProperties::No)
		.build()
		.await?;

	Ok(registry)
}

async fn setup_connection() -> AResult<(AccessibilityConnection, AccessibleProxy<'static>)> {
	// Enable accessibility for the session
	set_session_accessibility(true).await?;

	// Create a connection to the accessibility bus
	let connection = AccessibilityConnection::new().await?;

	// Get the root accessible object
	let root = get_registry_accessible(connection.connection()).await?;

	Ok((connection, root))
}

async fn frame_info<'a>(proxy: &AccessibleProxy<'a>) -> AResult<()> {
	eprintln!("frame info");
	Ok(())
}

// fn get_text<'a>(proxy: &AccessibleProxy<'a>) -> AResult<()> {
// 	proxy.
// 	eprintln!("get text");
// 	Ok(())
// }

async fn window_events_test() -> Result<(), Box<dyn Error>> {
	// Make sure applications with dynamic accessibility support do expose their AT-SPI2 interfaces.
	if let Err(e) = atspi_connection::set_session_accessibility(true)
		// .instrument(tracing::info_span!("setting accessibility enabled flag"))
		.await
	{
		eprintln!("Could not set AT-SPI2 IsEnabled property because: {}", e);
	}
	let (atspi, root) = setup_connection().await?;
	// let atspi = AccessibilityConnection::new().await?;

	// let root = get_registry_accessible(&atspi.connection()).await?;
	for child in root.get_children().await? {
		let proxy = child.into_accessible_proxy(atspi.connection()).await?;
		let ifs = proxy.get_interfaces().await?;
		eprintln!("interfaces: {ifs:?}");
		if let Ok(application) = proxy.get_application().await {
			eprintln!("application: {application:?}");
			// let role = proxy.get_role().await?;
			// let state = proxy.get_state().await?;
			// eprintln!("{role:?} {state:?}");

			if let Ok(children) = proxy.get_children().await {
				for child in children {
					let child_proxy = child.into_accessible_proxy(atspi.connection()).await?;
					let role = child_proxy.get_role().await?;
					let state = child_proxy.get_state().await?;
					eprintln!("{role:?} {state:?}");
					if state.contains(State::Active) {
						eprintln!(" child is active");
						let ifs = child_proxy.get_interfaces().await?;
						eprintln!("interfaces: {ifs:?}");
						let mut proxies = child_proxy.proxies().await?;
						let ht = proxies.text()?;
						// WORKAROUND atspi_connection bug
						let ht = TextProxy::builder(&atspi.connection())
							.destination(ht.inner().destination())?
							.path(ht.inner().path())?
							.build()
							.await?;
						// let text = ht.get_text(1, atspi::CoordType::Window).await?;
						match ht.get_text(0, 9999).await {
							Ok(_full_text) => eprintln!("text: {_full_text:?}"),
							Err(e) => eprintln!("Error getting text: {e}"),
						}
						// proxies.collection()?.get_matches(ObjectMatchRule::builder().interfaces(interfaces, mt), sortby, count, traverse)
					}
				}
			}
		}
	}
	return Ok(());
	// atspi.connection().object_server().at(path, iface)

	atspi.register_event::<WindowEvents>().await?;
	let events = atspi.event_stream();
	tokio::pin!(events);

	println!("Monitoring events (press Ctrl+C to exit)...");
	// Monitor the events
	while let Some(Ok(ev)) = events.next().await {
		println!("Received event {:?}", ev);
		if let Ok(wnd_active) = ActivateEvent::try_from(ev) {
			let able = wnd_active
				.item
				.into_accessible_proxy(atspi.connection())
				.await?;
			let role = able.get_role().await?;
			let state = able.get_state().await?;
			eprintln!("{role:?} {state:?}");
		}
		continue;
	}

	Ok(())
}

// FIXME: separate this all
// cargo test -- --nocapture
#[test]
fn main() -> Result<(), Box<dyn Error>> {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_io()
		.max_blocking_threads(1)
		.worker_threads(1)
		.build()
		.unwrap();
	let res = rt.block_on(window_events_test())?;
	Ok(res)
}
