//! # Buffer cursors
//!
//! ## Overview
//!
//! This module contains the types and logic for representing cursors and their manipulations
//! within a buffer.
//!
use std::cmp::{Ord, Ordering, PartialOrd};

use super::base::Wrappable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub(crate) xgoal: usize,
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub enum CursorAdjustment {
    Column {
        line: usize,
        column_start: usize,
        amt_line: isize,
        amt_col: isize,
    },
    Line {
        line_start: usize,
        line_end: usize,
        amount: isize,
        amount_after: isize,
    },
}

impl Cursor {
    /// Create a new cursor.
    pub fn new(line: usize, column: usize) -> Self {
        Cursor { xgoal: column, x: column, y: line }
    }

    /// Zero out this cursor's line and column.
    pub fn zero(&mut self) {
        self.xgoal = 0;
        self.x = 0;
        self.y = 0;
    }

    /// Set the column for this cursor.
    pub fn set_x(&mut self, x: usize) {
        self.x = x;
        self.xgoal = x;
    }

    /// Set the line for this cursor.
    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    /// Move this cursor to the left by offset columns.
    pub fn left(&mut self, off: usize) {
        self.x = self.x.saturating_sub(off);
        self.xgoal = self.x;
    }

    /// Move this cursor to the right by offset columns.
    pub fn right(&mut self, off: usize) {
        self.x = self.x.saturating_add(off);
        self.xgoal = self.x;
    }

    /// Move this cursor down by offset lines.
    pub fn down(&mut self, off: usize) {
        self.y = self.y.saturating_add(off);
    }

    /// Move this cursor up by offset lines.
    pub fn up(&mut self, off: usize) {
        self.y = self.y.saturating_sub(off);
    }

    fn adjust_x(&mut self, off: isize) {
        let abs = off.unsigned_abs();

        if off < 0 {
            self.left(abs);
        } else {
            self.right(abs);
        }
    }

    fn adjust_y(&mut self, off: isize) {
        let abs = off.unsigned_abs();

        if off < 0 {
            self.up(abs);
        } else {
            self.down(abs);
        }
    }

    /// Apply a [CursorAdjustment] to this cursor.
    pub fn adjust(&mut self, adj: &CursorAdjustment) {
        match adj {
            CursorAdjustment::Line { line_start, line_end, amount, amount_after } => {
                if self.y >= *line_start && self.y <= *line_end {
                    if *amount == isize::MAX {
                        self.zero();
                    } else {
                        self.adjust_y(*amount);
                    }
                } else if *amount_after != 0 && self.y > *line_end {
                    self.adjust_y(*amount_after);
                }
            },
            CursorAdjustment::Column { line, column_start, amt_line, amt_col } => {
                if self.y == *line && self.x >= *column_start {
                    self.adjust_y(*amt_line);
                    self.adjust_x(*amt_col);
                }
            },
        }
    }

    fn compare(&self, other: &Cursor) -> Ordering {
        let ycmp = self.y.cmp(&other.y);

        if ycmp != Ordering::Equal {
            return ycmp;
        }

        let xcmp = self.x.cmp(&other.x);

        if xcmp != Ordering::Equal {
            return xcmp;
        }

        self.xgoal.cmp(&other.xgoal)
    }
}

impl Wrappable for Cursor {
    fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.set_x(0);
        }
    }
}

impl Default for Cursor {
    fn default() -> Cursor {
        Cursor { xgoal: 0, x: 0, y: 0 }
    }
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Cursor) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Cursor {
    fn cmp(&self, other: &Cursor) -> Ordering {
        self.compare(other)
    }
}
