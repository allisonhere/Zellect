use crate::state::State;

pub fn render(state: &State, rows: usize, cols: usize) {
    if let Some(err) = &state.error {
        println!("\x1b[1;31mError:\x1b[0m {}", err);
        return;
    }

    if !state.permissions_granted || !state.session_update_received {
        let msg = if !state.permissions_granted {
            "Waiting for permission..."
        } else {
            "Loading layouts..."
        };
        let pad = cols.saturating_sub(msg.len()) / 2;
        let vpad = rows / 2;
        for _ in 0..vpad {
            println!();
        }
        println!("{}{}", " ".repeat(pad), msg);
        return;
    }

    if state.layouts.is_empty() {
        println!("\x1b[33mNo layouts found.\x1b[0m");
        println!("Place .kdl files in your zellij layouts directory.");
        return;
    }

    // Title bar
    let title = " Zellect — Add Layout To Current Session ";
    let title_pad = cols.saturating_sub(title.len()) / 2;
    println!(
        "{}\x1b[1;36m{}\x1b[0m",
        " ".repeat(title_pad),
        title
    );
    println!("{}", "─".repeat(cols));

    // Layout list
    let list_rows = rows.saturating_sub(4); // title + divider + help + padding
    let total = state.layouts.len();
    let half = list_rows / 2;
    let start = if state.selected >= half {
        (state.selected - half).min(total.saturating_sub(list_rows))
    } else {
        0
    };
    let end = (start + list_rows).min(total);

    for i in start..end {
        let (name, _) = &state.layouts[i];
        if i == state.selected {
            println!("\x1b[1;7m > {:<width$}\x1b[0m", name, width = cols.saturating_sub(3));
        } else {
            println!("   {}", name);
        }
    }

    // Fill remaining rows
    let shown = end - start;
    for _ in shown..list_rows {
        println!();
    }

    // Help bar
    println!("{}", "─".repeat(cols));
    println!(
        " \x1b[2m↑↓/jk\x1b[0m navigate  \x1b[2m<Enter>\x1b[0m add layout as new tab(s)  \x1b[2mq/Esc\x1b[0m close"
    );
}
