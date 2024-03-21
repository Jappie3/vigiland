use env_logger;
use log::{debug, error, info, warn};
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use wayland_client::{
    protocol::{
        wl_compositor, wl_registry,
        wl_surface,
    },
    Connection, Dispatch, Proxy,
};
use wayland_protocols::wp::idle_inhibit::zv1::client::{
    zwp_idle_inhibit_manager_v1::ZwpIdleInhibitManagerV1, zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1,
};

#[derive(Default)]
struct AppData {
    compositor: Option<(wl_compositor::WlCompositor, u32)>,
    surface: Option<wl_surface::WlSurface>,
    inhibit_manager: Option<(ZwpIdleInhibitManagerV1, u32)>,
    inhibitor: Option<ZwpIdleInhibitorV1>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _connection: &wayland_client::Connection,
        queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                debug!("| Received wl_registry::Event::Global: {interface} v{version}");
                if interface == wl_compositor::WlCompositor::interface().name
                    && state.compositor.is_none()
                {
                    // wl_compositor
                    info!("> Bound: {interface} v{version}");
                    let compositor: wl_compositor::WlCompositor =
                        proxy.bind(name, version, queue_handle, ());
                    state.surface = Some(compositor.create_surface(&queue_handle, ()));
                    state.compositor = Some((compositor, name));
                } else if interface == ZwpIdleInhibitManagerV1::interface().name {
                    // zwp_idle_inhibit_manager
                    info!("> Bound: {interface} v{version}");
                    state.inhibit_manager =
                        Some((proxy.bind(name, version, queue_handle, ()), name));
                };
            }
            wl_registry::Event::GlobalRemove { name } => {
                debug!("| Received wl_registry::Event::GlobalRemove");
                if let Some((_, compositor_name)) = &state.compositor {
                    if name == *compositor_name {
                        warn!("WlCompositor was removed");
                        state.compositor = None;
                        state.surface = None;
                    }
                } else if let Some((_, inhibit_manager_name)) = &state.inhibit_manager {
                    if name == *inhibit_manager_name {
                        warn!("ZwpIdleInhibitManagerV1 was removed");
                        state.inhibit_manager = None;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &wl_compositor::WlCompositor,
        _event: <wl_compositor::WlCompositor as Proxy>::Event,
        _data: &(),
        _connection: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &wl_surface::WlSurface,
        _event: <wl_surface::WlSurface as Proxy>::Event,
        _data: &(),
        _connection: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpIdleInhibitManagerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpIdleInhibitManagerV1,
        _event: <ZwpIdleInhibitManagerV1 as Proxy>::Event,
        _data: &(),
        _connection: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpIdleInhibitorV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpIdleInhibitorV1,
        _event: <ZwpIdleInhibitorV1 as Proxy>::Event,
        _data: &(),
        _connection: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        info!("Ctrl+C pressed, exiting...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting handler");

    let connection = Connection::connect_to_env().unwrap();
    let mut event_queue = connection.new_event_queue();
    let queue_handle = event_queue.handle();
    let display = connection.display();
    let _registry = display.get_registry(&queue_handle, ());
    let mut state = AppData::default();

    event_queue.roundtrip(&mut state).unwrap();

    let Some((inhibit_manager, _)) = &state.inhibit_manager else {
        error!("No ZwpIdleInhibitManagerV1 loaded");
        return Ok(());
    };
    let Some(surface) = &state.surface else {
        error!("No WlSurface loaded");
        return Ok(());
    };
    // create idle inhibitor
    state.inhibitor = Some(inhibit_manager.create_inhibitor(surface, &queue_handle, ()));
    event_queue.roundtrip(&mut state).unwrap();

    // wait for exit
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    let Some((inhibit_manager, _)) = &state.inhibit_manager else {
        error!("No ZwpIdleInhibitManagerV1 loaded");
        return Ok(());
    };
    let Some(inhibitor) = &state.inhibitor else {
        error!("No ZwpIdleInhibitorV1 loaded");
        return Ok(());
    };
    // cleanup, destroy inhibitor & inhibit manager
    inhibitor.destroy();
    inhibit_manager.destroy();
    event_queue.roundtrip(&mut state).unwrap();

    Ok(())
}
