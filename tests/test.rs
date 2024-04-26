use polite::Polite;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use whimsy::prelude::Command;
use winit::keyboard::ModifiersState;

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

    info!("Parsing modifiers.");
    parses_modifier()?;
    info!("Parsing modifiers successful.");

    info!("Parsing commands.");
    parses_command()?;
    info!("Parsing commands successful.");

    Ok(())
}

fn parses_modifier() -> Polite<()> {
    let c1 = "<cr> + a";
    let c2 = "<control> + b";
    let c3 = "<alt> + c";
    let c4 = "<shift> + d";
    let c7 = "<super> + g";
    let c8 = "<other> + h";
    let (key, mods) = Command::parse_mods(c1)?;
    assert_eq!(key, "a");
    assert_eq!(mods, Some(ModifiersState::CONTROL));
    let (key, mods) = Command::parse_mods(c2)?;
    assert_eq!(key, "b");
    assert_eq!(mods, Some(ModifiersState::CONTROL));
    let (key, mods) = Command::parse_mods(c3)?;
    assert_eq!(key, "c");
    assert_eq!(mods, Some(ModifiersState::ALT));
    let (key, mods) = Command::parse_mods(c4)?;
    assert_eq!(key, "d");
    assert_eq!(mods, Some(ModifiersState::SHIFT));
    let (key, mods) = Command::parse_mods(c7)?;
    assert_eq!(key, "g");
    assert_eq!(mods, Some(ModifiersState::SUPER));
    let (key, mods) = Command::parse_mods(c8)?;
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
    let (_, opt) = Command::parse_str(c1)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("a", &Some(ModifiersState::CONTROL)));
    }
    let (_, opt) = Command::parse_str(c2)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("b", &Some(ModifiersState::CONTROL)));
    }
    let (_, opt) = Command::parse_str(c3)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("c", &Some(ModifiersState::ALT)));
    }
    let (_, opt) = Command::parse_str(c4)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("d", &Some(ModifiersState::SHIFT)));
    }
    let (_, opt) = Command::parse_str(c7)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("g", &Some(ModifiersState::SUPER)));
    }
    let (_, opt) = Command::parse_str(c8)?;
    if let Some(c) = opt {
        assert_eq!(c, Command::new("h", &None));
    }

    Ok(())
}
