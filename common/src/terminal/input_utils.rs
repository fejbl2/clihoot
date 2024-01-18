use crossterm::event::KeyCode;

pub fn move_in_list(selected: &mut usize, list_size: usize, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Down | KeyCode::Char('s') => {
            *selected = (*selected + 1) % list_size;
            true
        }
        KeyCode::Up | KeyCode::Char('w') => {
            if *selected == 0 {
                *selected = list_size.saturating_sub(1);
            } else {
                *selected -= 1;
            }
            true
        }
        _ => false,
    }
}
