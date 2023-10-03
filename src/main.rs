use ncurses::{
    addstr, attroff, attron, endwin, getch, init_pair, initscr, mv, refresh, start_color,
    COLOR_BLACK, COLOR_WHITE, COLOR_PAIR, use_default_colors, noecho, curs_set, CURSOR_VISIBILITY,
};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    start_color();
    use_default_colors();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    main_loop();
    endwin();
}

fn main_loop() {
    let todos = vec![
        "write todo app",
        "make cup of tea",
        "buy a bread",
    ];
    let mut cur_pos = 0;
    loop {
        for (i, todo) in todos.iter().enumerate() {
            let pair = if cur_pos == i {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };

            attron(COLOR_PAIR(pair));
            mv(i as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
        refresh();
        let key = getch();
        match key as u8 as char {
            'q' => break,
            'j' => {
                cur_pos = std::cmp::min(cur_pos + 1, todos.len() - 1);
            }
            'k' => {
                if cur_pos == 0 {
                    continue;
                }
                cur_pos = std::cmp::max(cur_pos - 1, 0);
            }
            _ => {}
        }
    }
}
