use std::mem::size_of;
// I assume you have a move.rs file.
// If you call the file "move.rs", you must import it as r#move because "move" is a keyword.
use crate::r#move::Move;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
    Empty = 0,
    Exact = 1,
    Alpha = 2,
    Beta = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TTEntry {
    pub key: u64,
    pub bm: Move, // u16
    pub score: i32,
    pub depth: u8,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspositionTable {
    pub entries: Vec<TTEntry>,
    pub size: usize,
}

impl Default for TTEntry {
    fn default() -> Self {
        Self {
            key: 0,
            score: 0,
            bm: Move(0_u16),
            depth: 0,
            node_type: NodeType::Empty,
        }
    }
}

impl TranspositionTable {
    pub fn new(mb_size: usize) -> Self {
        let entry_size = size_of::<TTEntry>();
        // Calculate how many entries fit into the given MB size
        let target_count = (mb_size * 1024 * 1024) / entry_size;

        // Round down to nearest power of 2 for fast indexing (using & instead of %)
        let size = if target_count == 0 {
            1
        } else {
            target_count.next_power_of_two() >> 1
        };

        Self {
            entries: vec![TTEntry::default(); size],
            size,
        }
    }

    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = TTEntry::default();
        }
    }

    pub fn probe(&self, key: u64) -> Option<TTEntry> {
        // Fast modulo using bitwise AND (works because size is power of 2)
        let index = (key as usize) & (self.size - 1);
        let entry = self.entries[index];

        // Return entry only if keys match and it's not empty
        if entry.key == key && entry.node_type != NodeType::Empty {
            Some(entry)
        } else {
            None
        }
    }

    pub fn store(&mut self, key: u64, score: i32, depth: u8, flag: NodeType, best_move: Move) {
        let index = (key as usize) & (self.size - 1);
        let entry = &mut self.entries[index];

        // Replacement Strategy:
        // 1. Slot is empty
        // 2. Collision (different position) -> Always replace (new position is likely more relevant)
        // 3. Same position -> Replace only if new depth is better or equal
        if entry.node_type == NodeType::Empty || entry.key != key || depth >= entry.depth {
            entry.key = key;
            entry.score = score;
            entry.depth = depth;
            entry.node_type = flag;
            entry.bm = best_move;
        }
    }
}