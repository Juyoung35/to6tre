mod minesweeper {
    
}

enum TentsAndTreesCell {
    #[default]
    Empty {
        l_click: Tent,
        r_click: Flagged,
        color: WHITE,
    },
    Flagged {
        l_click: Tent,
        r_click: Empty,
        color: WHITE,
    },
    #[random(0)]
    Tree {
        color: GREEN,
        sprite: "assets/tree.svg",
    },
    Tent {
        l_click: Empty,
        r_click: Flagged,
        color: GREEN,
        sprite: "assets/tent.svg",
    },
}