#!/usr/bin/env python3

def generate_pawn_attacks():
    """
    Generates a list of 64 u64 bitboards for each color (White, Black),
    where each bitboard represents the squares a pawn can attack from
    a given source square.
    Uses Little-Endian File-Rank (LEFR) mapping: A1=0, H1=7, A2=8, ..., H8=63.

    Returns:
        [[u64; 64]; 2]  (represented as a list of two lists)
        Index 0: White attacks
        Index 1: Black attacks
    """

    # all_attacks[0] = White, all_attacks[1] = Black
    all_attacks = [[], []]

    for sq in range(64):
        rank = sq // 8
        file = sq % 8

        white_attacks_bb = 0
        black_attacks_bb = 0

        # --- White Attacks (Index 0) ---
        # Pawns attack "up" the board (increasing square index)
        # We only generate attacks if the pawn is not on the 8th rank
        if rank < 7:
            # Attack Up-Left (e.g., B2 -> A3)
            # Not possible if pawn is on the A-file
            if file > 0:
                white_attacks_bb |= (1 << (sq + 7))

            # Attack Up-Right (e.g., B2 -> C3)
            # Not possible if pawn is on the H-file
            if file < 7:
                white_attacks_bb |= (1 << (sq + 9))

        # --- Black Attacks (Index 1) ---
        # Pawns attack "down" the board (decreasing square index)
        # We only generate attacks if the pawn is not on the 1st rank
        if rank > 0:
            # Attack Down-Left (e.g., G7 -> F6)
            # Not possible if pawn is on the A-file
            if file > 0:
                black_attacks_bb |= (1 << (sq - 9))

            # Attack Down-Right (e.g., G7 -> H6)
            # Not possible if pawn is on the H-file
            if file < 7:
                black_attacks_bb |= (1 << (sq - 7))

        all_attacks[0].append(white_attacks_bb)
        all_attacks[1].append(black_attacks_bb)

    return all_attacks

def print_rust_array(attacks_by_color, const_name):
    """
    Prints the list of attack bitboards as a nested Rust array.
    """
    print(f"pub const {const_name}: [[u64; 64]; 2] = [")

    # --- Print White Attacks ---
    print("    [ // Color 0: White")
    for i, attacks_bb in enumerate(attacks_by_color[0]):
        raw_binary_string = f"{attacks_bb:064b}"
        chunks = [raw_binary_string[i:i+8] for i in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"
        print(f"        {binary_string}, // Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")
    print("    ],")

    # --- Print Black Attacks ---
    print("    [ // Color 1: Black")
    for i, attacks_bb in enumerate(attacks_by_color[1]):
        raw_binary_string = f"{attacks_bb:064b}"
        chunks = [raw_binary_string[i:i+8] for i in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"
        print(f"        {binary_string}, // Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")
    print("    ]")

    print("];")

if __name__ == "__main__":
    attacks = generate_pawn_attacks()
    print_rust_array(attacks, "PAWN_ATTACKS")