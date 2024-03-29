use uuid::Uuid;

use common::terminal::widgets::choice::{Grid, Item, SelectorState};

fn one_row_fixture(uuids: &[Uuid]) -> Grid {
    let row = uuids
        .iter()
        .map(|id| Some(Item::new(id.to_string(), false, *id)))
        .collect();

    Grid::new(vec![row])
}

fn multiple_row_fixture(uuids: &[Vec<Uuid>]) -> Grid {
    Grid::new(
        uuids
            .iter()
            .map(|row| {
                row.iter()
                    .map(|id| Some(Item::new(id.to_string(), false, *id)))
                    .collect()
            })
            .collect(),
    )
}

#[test]
fn test_one_row_move_around() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let grid = one_row_fixture(&uuids);

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0]));

    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0]));

    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0]));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuids[2]));

    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0]));
}

#[test]
fn test_one_row_toggle_selection_multichoice() {
    let is_multichoice = true;
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let grid = one_row_fixture(&uuids);

    let mut state = SelectorState::default();
    assert!(state.selected().is_empty());

    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0]));

    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.is_empty());

    state.move_right(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));

    state.move_right(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 2 && selected.contains(&uuids[1]) && selected.contains(&uuids[2]));

    state.move_left(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[2]));

    state.toggle_selection(&grid, is_multichoice);
    state.move_left(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(
        selected.len() == 3
            && selected.contains(&uuids[0])
            && selected.contains(&uuids[1])
            && selected.contains(&uuids[2])
    );
}

#[test]
fn test_one_row_toggle_selection_singlechoice() {
    let is_multichoice = false;
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let grid = one_row_fixture(&uuids);

    let mut state = SelectorState::default();
    assert!(state.selected().is_empty());

    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0]));

    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.is_empty());

    state.move_right(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));

    state.move_right(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[2]));

    state.move_left(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));

    state.toggle_selection(&grid, is_multichoice);
    state.move_left(&grid);
    state.toggle_selection(&grid, is_multichoice);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0]));
}

#[test]
fn test_classic_4_choices_move_around() {
    let uuids = vec![
        vec![Uuid::new_v4(), Uuid::new_v4()],
        vec![Uuid::new_v4(), Uuid::new_v4()],
    ];

    let grid = multiple_row_fixture(&uuids);

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    state.move_down(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0][0]));

    state.move_up(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));

    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0][0]));

    state.move_down(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][1]));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));

    state.move_left(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][1]));

    state.move_left(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));
}

#[test]
fn test_classic_4_choices_toggle_selection() {
    let uuids = vec![
        vec![Uuid::new_v4(), Uuid::new_v4()],
        vec![Uuid::new_v4(), Uuid::new_v4()],
    ];
    let grid = multiple_row_fixture(&uuids);

    let mut state = SelectorState::default();
    assert!(state.selected().is_empty());

    state.toggle_selection(&grid, true);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0][0]));

    state.toggle_selection(&grid, true);
    let selected = state.selected();
    assert!(selected.is_empty());

    state.move_right(&grid);
    state.toggle_selection(&grid, true);
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[0][1]));

    state.move_right(&grid);
    state.toggle_selection(&grid, true);
    let selected = state.selected();
    assert!(
        selected.len() == 2 && selected.contains(&uuids[0][0]) && selected.contains(&uuids[0][1])
    );

    state.move_down(&grid);
    state.toggle_selection(&grid, true);
    let selected = state.selected();
    assert!(
        selected.len() == 3
            && selected.contains(&uuids[0][0])
            && selected.contains(&uuids[0][1])
            && selected.contains(&uuids[1][0])
    );

    state.move_right(&grid);
    state.toggle_selection(&grid, true);
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

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0][0]));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0][0]));

    state.move_down(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][0]));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][1]));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuids[1][2]));

    // correctly change the state.col when moving to row with less items
    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[0][0]));
}

#[test]
fn test_reconfigure_grid() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

    // single row with 3 choices
    let grid = Grid::new(vec![vec![
        Some(Item::new(uuids[0].to_string(), false, uuids[0])),
        Some(Item::new(uuids[1].to_string(), false, uuids[1])),
        Some(Item::new(uuids[2].to_string(), false, uuids[2])),
    ]]);

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // move to the middle item
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));

    // rearange the grid to be one single column with three rows
    let grid = Grid::new(vec![
        vec![Some(Item::new(uuids[0].to_string(), false, uuids[0]))],
        vec![Some(Item::new(uuids[1].to_string(), false, uuids[1]))],
        vec![Some(Item::new(uuids[2].to_string(), false, uuids[2]))],
    ]);

    state.move_to_last_known_choice(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));

    // rearange back to the original config
    let grid = Grid::new(vec![vec![
        Some(Item::new(uuids[0].to_string(), false, uuids[0])),
        Some(Item::new(uuids[1].to_string(), false, uuids[1])),
        Some(Item::new(uuids[2].to_string(), false, uuids[2])),
    ]]);

    // try to select item under the cursor, without calling the move_to_last_known_choice function
    state.toggle_selection(&grid, true);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));
}

#[test]
fn test_grid_changed_completely() {
    let uuid_old_1 = Uuid::new_v4();
    let uuid_old_2 = Uuid::new_v4();

    // single row with 1 choice
    let grid = Grid::new(vec![vec![
        Some(Item::new("foo".to_string(), false, uuid_old_1)),
        Some(Item::new("bar".to_string(), false, uuid_old_2)),
    ]]);

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // move to the right
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_old_2));

    // completely change the grid, this should not happen when using the
    // widget, if the user uses the widget like an inteligent human being
    let uuid_new_1 = Uuid::new_v4();
    let uuid_new_2 = Uuid::new_v4();

    let grid = Grid::new(vec![vec![
        Some(Item::new("foo".to_string(), false, uuid_new_1)),
        Some(Item::new("bar".to_string(), false, uuid_new_2)),
    ]]);

    // we should be back at [0, 0] so we can somehow move around again
    state.move_to_last_known_choice(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
}

#[test]
fn test_empty_grid() {
    let grid = Grid::new(Vec::new());
    assert!(grid.is_empty());

    let mut state = SelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // nothing should happen
    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());
}

#[test]
fn test_grid_empty_line() {
    let grid = Grid::new(vec![vec![]]);
    assert!(grid.is_empty());

    let mut state = SelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // nothing should happen
    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());
}

#[test]
fn test_only_none_line() {
    let grid = Grid::new(vec![vec![None]]);
    assert!(grid.is_empty());

    let mut state = SelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // nothing should happen
    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());
}

#[test]
fn test_only_none_matrix() {
    let grid = Grid::new(vec![vec![None, None], vec![None, None]]);
    assert!(grid.is_empty());

    let mut state = SelectorState::default();

    // nothing should happen
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // nothing should happen
    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());
}

#[test]
fn test_row_with_none() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![vec![
        Some(Item::new("test".to_string(), false, uuid_1)),
        None,
        Some(Item::new("test".to_string(), false, uuid_2)),
        None,
    ]]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_down(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_4_with_none() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![
        vec![Some(Item::new("test".to_string(), false, uuid_1)), None],
        vec![None, Some(Item::new("test".to_string(), false, uuid_2))],
    ]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_down(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_left(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_4_with_none_mirrored() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![
        vec![None, Some(Item::new("test".to_string(), false, uuid_1))],
        vec![Some(Item::new("test".to_string(), false, uuid_2)), None],
    ]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_down(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_left(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_only_none_line_in_middle() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![
        vec![Some(Item::new("test".to_string(), false, uuid_1)), None],
        vec![None, None],
        vec![None, Some(Item::new("test".to_string(), false, uuid_2))],
    ]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_left(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));

    state.move_down(&grid);
    assert_eq!(state.row(), 2);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_right(&grid);
    assert_eq!(state.row(), 2);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_left(&grid);
    assert_eq!(state.row(), 2);
    assert_eq!(state.col(), 1);
    assert_eq!(state.last_under_cursor(), Some(uuid_2));

    state.move_up(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_start_on_none_row_move_right() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![
        vec![None, None],
        vec![
            Some(Item::new("test".to_string(), false, uuid_1)),
            Some(Item::new("test".to_string(), false, uuid_2)),
        ],
    ]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_start_on_none_row_move_left() {
    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let grid = Grid::new(vec![
        vec![None, None],
        vec![
            Some(Item::new("test".to_string(), false, uuid_1)),
            Some(Item::new("test".to_string(), false, uuid_2)),
        ],
    ]);

    let mut state = SelectorState::default();
    state.move_right(&grid);
    assert_eq!(state.row(), 1);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuid_1));
}

#[test]
fn test_reconfigure_with_none() {
    let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

    // single row with 3 choices
    let grid = Grid::new(vec![vec![
        Some(Item::new(uuids[0].to_string(), false, uuids[0])),
        None,
        Some(Item::new(uuids[1].to_string(), false, uuids[1])),
        None,
        Some(Item::new(uuids[2].to_string(), false, uuids[2])),
    ]]);

    let mut state = SelectorState::default();
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 0);
    assert!(state.last_under_cursor().is_none());

    // move to the middle item
    state.move_right(&grid);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));

    // rearange the grid to be one single column with three rows
    let grid = Grid::new(vec![
        vec![Some(Item::new(uuids[0].to_string(), false, uuids[0]))],
        vec![None],
        vec![Some(Item::new(uuids[1].to_string(), false, uuids[1]))],
        vec![None],
        vec![Some(Item::new(uuids[2].to_string(), false, uuids[2]))],
    ]);

    state.move_to_last_known_choice(&grid);
    assert_eq!(state.row(), 2);
    assert_eq!(state.col(), 0);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));

    // rearange back to the original config
    let grid = Grid::new(vec![vec![
        Some(Item::new(uuids[0].to_string(), false, uuids[0])),
        None,
        Some(Item::new(uuids[1].to_string(), false, uuids[1])),
        None,
        Some(Item::new(uuids[2].to_string(), false, uuids[2])),
    ]]);

    // try to select item under the cursor, without calling the move_to_last_known_choice function
    state.toggle_selection(&grid, true);
    assert_eq!(state.row(), 0);
    assert_eq!(state.col(), 2);
    assert_eq!(state.last_under_cursor(), Some(uuids[1]));
    let selected = state.selected();
    assert!(selected.len() == 1 && selected.contains(&uuids[1]));
}
