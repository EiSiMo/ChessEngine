#!/usr/bin/env python3

def generate_pawn_attacked_by():
    """
    Generates a list of 64 u64 bitboards for each color (White, Black),
    where each bitboard represents the squares FROM WHICH a pawn
    can attack a given TARGET square.
    Uses Little-Endian File-Rank (LEFR) mapping: A1=0, H1=7, A2=8, ..., H8=63.

    Returns:
        [[u64; 64]; 2]  (represented as a list of two lists)
        Index 0: White attackers (squares where a White pawn attacks the target)
        Index 1: Black attackers (squares where a Black pawn attacks the target)
    """

    # all_attacks[0] = White, all_attacks[1] = Black
    all_attacks = [[], []]

    for sq in range(64):
        rank = sq // 8
        file = sq % 8

        white_attackers_bb = 0
        black_attackers_bb = 0

        # --- White Attackers (Index 0) ---
        # A white pawn attacks "up" the board to reach the target square 'sq'.
        # This means the attacking pawn must be "below" 'sq'.
        # We only check if the target is not on the 1st rank
        if rank > 0:
            # Attacker from Down-Left (relative to target sq)
            # Not possible if target is on the A-file
            if file > 0:
                white_attackers_bb |= (1 << (sq - 9))

            # Attacker from Down-Right (relative to target sq)
            # Not possible if target is on the H-file
            if file < 7:
                white_attackers_bb |= (1 << (sq - 7))

        # --- Black Attackers (Index 1) ---
        # A black pawn attacks "down" the board to reach the target square 'sq'.
        # This means the attacking pawn must be "above" 'sq'.
        # We only check if the target is not on the 8th rank
        if rank < 7:
            # Attacker from Up-Left (relative to target sq)
            # Not possible if target is on the A-file
            if file > 0:
                black_attackers_bb |= (1 << (sq + 7))

            # Attacker from Up-Right (relative to target sq)
            # Not possible if target is on the H-file
            if file < 7:
                black_attackers_bb |= (1 << (sq + 9))

        all_attacks[0].append(white_attackers_bb)
        all_attacks[1].append(black_attackers_bb)

    return all_attacks

def print_rust_array(attacks_by_color, const_name):
    """
    Prints the list of attack bitboards as a nested Rust array.
    """
    print(f"pub const {const_name}: [[u64; 64]; 2] = [")

    # --- Print White Attacks ---
    print("    [ // Color 0: White (Squares a White pawn attacks FROM)")
    for i, attacks_bb in enumerate(attacks_by_color[0]):
        raw_binary_string = f"{attacks_bb:064b}"
        chunks = [raw_binary_string[i:i+8] for i in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"
        print(f"        {binary_string}, // Target Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")
    print("    ],")

    # --- Print Black Attacks ---
    print("    [ // Color 1: Black (Squares a Black pawn attacks FROM)")
    for i, attacks_bb in enumerate(attacks_by_color[1]):
        raw_binary_string = f"{attacks_bb:064b}"
        chunks = [raw_binary_string[i:i+8] for i in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"
        print(f"        {binary_string}, // Target Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")
    print("    ]")

    print("];")

if __name__ == "__main__":
    attacks = generate_pawn_attacked_by()
    print_rust_array(attacks, "ATTACKING_PAWNS")