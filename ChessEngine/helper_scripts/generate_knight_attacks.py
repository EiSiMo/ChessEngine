#!/usr/bin/env python3

def generate_knight_attacks():
    """
    Generates a list of 64 u64 bitboards, where each bitboard represents
    the squares a knight can attack from a given source square.
    Uses Little-Endian File-Rank (LEFR) mapping: A1=0, H1=7, A2=8, ..., H8=63.
    """
    all_attacks = []

    # All 8 possible knight move offsets (delta_file, delta_rank)
    knight_moves = [
        (1, 2), (1, -2), (-1, 2), (-1, -2),
        (2, 1), (2, -1), (-2, 1), (-2, -1)
    ]

    for sq in range(64):
        attacks_bb = 0

        # Calculate rank and file for the source square
        rank = sq // 8
        file = sq % 8

        for df, dr in knight_moves:
            target_rank = rank + dr
            target_file = file + df

            # Check if the target square is on the board
            if 0 <= target_rank <= 7 and 0 <= target_file <= 7:
                # Convert target rank and file back to a square index
                target_sq = target_rank * 8 + target_file
                # Set the corresponding bit in the bitboard
                attacks_bb |= (1 << target_sq)

        all_attacks.append(attacks_bb)

    return all_attacks

def print_rust_array(attacks):
    """
    Prints the list of attack bitboards as a Rust array.
    """
    print("pub const KNIGHT_ATTACKS: [u64; 64] = [")

    for i, attacks_bb in enumerate(attacks):
        # Format as 64-bit zero-padded binary string
        raw_binary_string = f"{attacks_bb:064b}"

        # Insert underscores every 8 bits for readability
        chunks = [raw_binary_string[i:i+8] for i in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"

        # Correctly index the string 'ABCDEFGH'
        print(f"    {binary_string}, // Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")

    print("];")

if __name__ == "__main__":
    attacks = generate_knight_attacks()
    print_rust_array(attacks)