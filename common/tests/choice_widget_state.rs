use uuid::Uuid;

use common::terminal::widgets::choice::{ChoiceGrid, ChoiceItem, ChoiceSelectorState};

fn one_row_fixture(uuids: &[Uuid]) -> ChoiceGrid {
    let row = uuids
        .iter()
        .map(|id| ChoiceItem::new(id.to_string(), false, *id))
        .collect();

    ChoiceGrid::new(vec![row])
}

fn multiple_row_fixture(uuids: &[Vec<Uuid>]) -> ChoiceGrid {
    ChoiceGrid::new(
        uuids
            .iter()
            .map(|row| {
                row.iter()
                    .map(|id| ChoiceItem::new(id.to_string(), false, *id))
                    .collect()
            })
            .collect(),
    )
}

#[test]
fn test_one_row_move_around() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let grid = one_row_fixture(&uuids);

    let mut state = ChoiceSelectorState::default();
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());

    state.move_up(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0]));

    state.move_down(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0]));

    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1]));

    state.move_left(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0]));

    state.move_left(&grid);
    assert!(state.row() == 0 && state.col() == 2);
    assert!(state.last_under_cursor() == Some(uuids[2]));

    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0]));
}

#[test]
fn test_one_row_toggle_selection() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let grid = one_row_fixture(&uuids);

    let mut state = ChoiceSelectorState::default();
    assert!(state.selected().is_empty());

    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0]));

    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.is_empty());

    state.move_right(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));

    state.move_right(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 2 && selected.contains(&uuids[1]) && selected.contains(&uuids[2]));

    state.move_left(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[2]));

    state.toggle_selection(&grid);
    state.move_left(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(
        selected.len() == 3
            && selected.contains(&uuids[0])
            && selected.contains(&uuids[1])
            && selected.contains(&uuids[2])
    );
}

#[test]
fn test_classic_4_choices_move_around() {
    let uuids = vec![
        vec![Uuid::new_v4(), Uuid::new_v4()],
        vec![Uuid::new_v4(), Uuid::new_v4()],
    ];

    let grid = multiple_row_fixture(&uuids);

    let mut state = ChoiceSelectorState::default();
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());

    state.move_down(&grid);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));

    state.move_up(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0][0]));

    state.move_up(&grid);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));

    state.move_down(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0][0]));

    state.move_down(&grid);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));

    state.move_right(&grid);
    assert!(state.row() == 1 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1][1]));

    state.move_right(&grid);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));

    state.move_left(&grid);
    assert!(state.row() == 1 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1][1]));

    state.move_left(&grid);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));
}

#[test]
fn test_classic_4_choices_toggle_selection() {
    let uuids = vec![
        vec![Uuid::new_v4(), Uuid::new_v4()],
        vec![Uuid::new_v4(), Uuid::new_v4()],
    ];
    let grid = multiple_row_fixture(&uuids);

    let mut state = ChoiceSelectorState::default();
    assert!(state.selected().is_empty());

    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0][0]));

    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.is_empty());

    state.move_right(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0][1]));

    state.move_right(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(
        selected.len() == 2 && selected.contains(&uuids[0][0]) && selected.contains(&uuids[0][1])
    );

    state.move_down(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(
        selected.len() == 3
            && selected.contains(&uuids[0][0])
            && selected.contains(&uuids[0][1])
            && selected.contains(&uuids[1][0])
    );

    state.move_right(&grid);
    state.toggle_selection(&grid);
    let selected = state.selected();
    assert!(
        selected.len() == 4
            && selected.contains(&uuids[0][0])
            && selected.contains(&uuids[0][1])
            && selected.contains(&uuids[1][0])
            && selected.contains(&uuids[1][1])
    );
}

#[test]
fn test_random_grid_move_around() {
    let uuids = vec![
        vec![Uuid::new_v4()],
        vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
    ];
    let grid = multiple_row_fixture(&uuids);

    let mut state = ChoiceSelectorState::default();
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());

    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0][0]));

    state.move_left(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0][0]));

    state.move_down(&grid);
    println!("{:?}", state);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1][0]));

    state.move_right(&grid);
    assert!(state.row() == 1 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1][1]));

    state.move_right(&grid);
    assert!(state.row() == 1 && state.col() == 2);
    assert!(state.last_under_cursor() == Some(uuids[1][2]));

    // correctly change the state.col when moving to row with less items
    state.move_up(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[0][0]));
}

#[test]
fn test_reconfigure_grid() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

    // single row with 3 choices
    let grid = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(uuids[0].to_string(), false, uuids[0]),
        ChoiceItem::new(uuids[1].to_string(), false, uuids[1]),
        ChoiceItem::new(uuids[2].to_string(), false, uuids[2]),
    ]]);

    let mut state = ChoiceSelectorState::default();
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());

    // move to the middle item
    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1]));

    // rearange the grid to be one single column with three rows
    let grid = ChoiceGrid::new(vec![
        vec![ChoiceItem::new(uuids[0].to_string(), false, uuids[0])],
        vec![ChoiceItem::new(uuids[1].to_string(), false, uuids[1])],
        vec![ChoiceItem::new(uuids[2].to_string(), false, uuids[2])],
    ]);

    state.move_to_last_known_choice(&grid);
    println!("{:?}", state);
    assert!(state.row() == 1 && state.col() == 0);
    assert!(state.last_under_cursor() == Some(uuids[1]));

    // rearange back to the original config
    let grid = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(uuids[0].to_string(), false, uuids[0]),
        ChoiceItem::new(uuids[1].to_string(), false, uuids[1]),
        ChoiceItem::new(uuids[2].to_string(), false, uuids[2]),
    ]]);

    // try to select item under the cursor, without calling the move_to_last_known_choice function
    state.toggle_selection(&grid);
    assert!(state.row() == 0 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuids[1]));
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));
}

#[test]
fn test_grid_changed_completely() {
    let uuid_old_1 = Uuid::new_v4();
    let uuid_old_2 = Uuid::new_v4();

    // single row with 1 choice
    let grid = ChoiceGrid::new(vec![vec![
        ChoiceItem::new("foo".to_string(), false, uuid_old_1),
        ChoiceItem::new("bar".to_string(), false, uuid_old_2),
    ]]);

    let mut state = ChoiceSelectorState::default();
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());

    // move to the right
    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 1);
    assert!(state.last_under_cursor() == Some(uuid_old_2));

    // completely change the grid, this should not happen when using the
    // widget, if the user uses the widget like an inteligent human being
    let uuid_new_1 = Uuid::new_v4();
    let uuid_new_2 = Uuid::new_v4();

    let grid = ChoiceGrid::new(vec![vec![
        ChoiceItem::new("foo".to_string(), false, uuid_new_1),
        ChoiceItem::new("bar".to_string(), false, uuid_new_2),
    ]]);

    // we should be back at [0, 0] so we can somehow move around again
    state.move_to_last_known_choice(&grid);
    assert!(state.row() == 0 && state.col() == 0);
}

#[test]
fn test_empty_grid() {
    let grid = ChoiceGrid::new(Vec::new());
    assert!(grid.is_empty());

    let mut state = ChoiceSelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());
}

#[test]
fn test_grid_empty_line() {
    let grid = ChoiceGrid::new(vec![vec![]]);
    assert!(grid.is_empty());

    let mut state = ChoiceSelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert!(state.row() == 0 && state.col() == 0);
    assert!(state.last_under_cursor().is_none());
}
