use address::prelude::Portable;
use polite::Polite;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use whimsy::prelude::Command;
use whimsy::prelude::Modifiers;
use winit::keyboard::ModifiersState;

fn init_tracing() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "test=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
}

#[test]
fn observer() -> Polite<()> {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "test=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    info!("Parsing keys.");
    parses_keys()?;
    info!("Parsing keys successful.");

    info!("Parsing modifiers.");
    parses_modifier()?;
    info!("Parsing modifiers successful.");

    info!("Parsing commands.");
    parses_command()?;
    info!("Parsing commands successful.");

    Ok(())
}

fn parses_keys() -> Polite<()> {
    let mut input = ('a'..='z')
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let uppercase = ('A'..='Z')
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    input.extend(uppercase);

    for op in input {
        let cmd = Command::parse_cmd(&op)?;
        assert_eq!(cmd.key, op);
        tracing::trace!("Key {} parsed.", &cmd.key);
    }
    Ok(())
}

fn parses_modifier() -> Polite<()> {
    let c1 = "<cr> + a";
    let c2 = "<control> + b";
    let c3 = "<alt> + c";
    let c4 = "<shift> + d";
    let c7 = "<super> + g";
    let c8 = "<other> + h";
    let (key, mods) = Command::parse_mod(c1)?;
    assert_eq!(key, "a");
    if let Some(m) = mods {
        assert!(m.control_key());
    }
    let (key, mods) = Command::parse_mod(c2)?;
    assert_eq!(key, "b");
    if let Some(m) = mods {
        assert!(m.control_key());
    }
    let (key, mods) = Command::parse_mod(c3)?;
    assert_eq!(key, "c");
    if let Some(m) = mods {
        assert!(m.alt_key());
    }
    let (key, mods) = Command::parse_mod(c4)?;
    assert_eq!(key, "d");
    if let Some(m) = mods {
        assert!(m.shift_key());
    }
    let (key, mods) = Command::parse_mod(c7)?;
    assert_eq!(key, "g");
    if let Some(m) = mods {
        assert!(m.super_key());
    }
    let (key, mods) = Command::parse_mod(c8)?;
    assert_eq!(key, "h");
    assert_eq!(mods, None);
    Ok(())
}

fn parses_command() -> Polite<()> {
    let c1 = "<cr> + a";
    let c2 = "<control> + b";
    let c3 = "<alt> + c";
    let c4 = "<shift> + d";
    let c7 = "<super> + g";
    let c8 = "<other> + h";
    let mut comp = Modifiers::new();
    let (_, opt) = Command::parse_str(c8)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("h", &comp));
    }
    let (_, opt) = Command::parse_str(c1)?;
    comp.control_key = true;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("a", &comp));
    }
    let (_, opt) = Command::parse_str(c2)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("b", &comp));
    }
    let (_, opt) = Command::parse_str(c3)?;
    comp.control_key = false;
    comp.alt_key = true;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("c", &comp));
    }
    comp.alt_key = false;
    comp.shift_key = true;
    let (_, opt) = Command::parse_str(c4)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("d", &comp));
    }
    let (_, opt) = Command::parse_str(c7)?;
    comp.shift_key = false;
    comp.super_key = true;
    if let Some(c) = opt {
        assert_eq!(c, Command::with_modifier("g", &comp));
    }

    Ok(())
}

#[test]
fn import_addresses() -> Polite<()> {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "test=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    let sa = address::prelude::GrantsPassSpatialAddresses::from_csv("data/addresses.csv").unwrap();
    let sa = address::prelude::SpatialAddresses::from(&sa[..]);
    let sa = whimsy::prelude::AddressPoints::from(&sa);
    sa.save("data/addresses.data")?;
    Ok(())
}

#[test]
fn iterate_enum() -> Polite<()> {
    use strum::IntoEnumIterator;
    init_tracing();
    tracing::info!("Desired standard order.");
    let mut egui = whimsy::prelude::EguiAct::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    egui.pop();
    egui.iter().map(|x| tracing::info!("{x}")).for_each(drop);
    let mut named = whimsy::prelude::NamedAct::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    named.pop();
    named.iter().map(|x| tracing::info!("{x}")).for_each(drop);
    let app = whimsy::prelude::AppAct::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    app.iter().map(|x| tracing::info!("{x}")).for_each(drop);

    tracing::info!("Sorted order.");
    let mut egui = whimsy::prelude::EguiAct::iter().collect::<Vec<whimsy::prelude::EguiAct>>();
    egui.sort();
    let mut egui = egui.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    egui.pop();
    egui.iter().map(|x| tracing::info!("{x}")).for_each(drop);
    let mut named = whimsy::prelude::NamedAct::iter().collect::<Vec<whimsy::prelude::NamedAct>>();
    named.sort();
    let mut named = named.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    named.pop();
    named.iter().map(|x| tracing::info!("{x}")).for_each(drop);
    let mut app = whimsy::prelude::AppAct::iter().collect::<Vec<whimsy::prelude::AppAct>>();
    app.sort();
    let mut app = app.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    app.pop();
    app.iter().map(|x| tracing::info!("{x}")).for_each(drop);

    Ok(())
}
