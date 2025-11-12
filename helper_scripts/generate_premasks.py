#!/usr/bin/env python3

def generate_premasks():
    """
    Generiert relevante Belegungsmasken (relevant occupancy masks) für
    Türme (Rooks) und Läufer (Bishops) auf allen 64 Feldern.

    Diese Masken enthalten alle Felder zwischen der Figur und dem Rand,
    ABER AUSSCHLIESSLICH der Randfelder selbst.
    """
    all_rook_masks = []
    all_bishop_masks = []

    # Richtungen für beide Figurentypen
    rook_directions = [
        (1, 0),   # Hoch
        (-1, 0),  # Runter
        (0, 1),   # Rechts
        (0, -1)   # Links
    ]
    bishop_directions = [
        (1, 1),   # Hoch-Rechts
        (1, -1),  # Hoch-Links
        (-1, 1),  # Runter-Rechts
        (-1, -1)  # Runter-Links
    ]

    for sq in range(64):
        rook_mask_bb = 0
        bishop_mask_bb = 0

        rank = sq // 8
        file = sq % 8

        # --- 1. Turm (Rook) Masken-Generierung ---
        # (Dies ist die korrigierte Logik)
        for dr, df in rook_directions:
            target_rank = rank + dr
            target_file = file + df

            # Schleife, solange wir auf dem Brett sind (0-7)
            while 0 <= target_rank <= 7 and 0 <= target_file <= 7:
                is_relevant = False

                # Prüfen, ob das Feld *vor* dem Rand liegt.
                if df != 0: # Horizontale Bewegung
                    if 1 <= target_file <= 6: # Files 'b' bis 'g'
                        is_relevant = True
                elif dr != 0: # Vertikale Bewegung
                    if 1 <= target_rank <= 6: # Ranks 2 bis 7
                        is_relevant = True

                if is_relevant:
                    target_sq = target_rank * 8 + target_file
                    rook_mask_bb |= (1 << target_sq)

                # Zum nächsten Feld in dieser Richtung
                target_rank += dr
                target_file += df

        all_rook_masks.append(rook_mask_bb)

        # --- 2. Läufer (Bishop) Masken-Generierung ---
        # (Diese Logik war in deinem "Rook"-Skript und ist hier korrekt)
        for dr, df in bishop_directions:
            target_rank = rank + dr
            target_file = file + df

            # Schleife, solange wir *von allen* Rändern entfernt sind (1-6)
            while 1 <= target_rank <= 6 and 1 <= target_file <= 6:
                target_sq = target_rank * 8 + target_file
                bishop_mask_bb |= (1 << target_sq)

                # Zum nächsten Feld in dieser Richtung
                target_rank += dr
                target_file += df

        all_bishop_masks.append(bishop_mask_bb)

    # Gibt beide Listen als Tupel zurück
    return all_rook_masks, all_bishop_masks

def print_rust_array(attacks, const_name):
    """
    Gibt die Liste der Bitboards als Rust-Array aus.
    """
    print(f"pub const {const_name}: [u64; 64] = [")

    for i, attacks_bb in enumerate(attacks):
        # Formatieren als 64-Bit binärer String mit Nullen
        raw_binary_string = f"{attacks_bb:064b}"

        # Unterstriche alle 8 Bits zur Lesbarkeit einfügen
        chunks = [raw_binary_string[j:j+8] for j in range(0, 64, 8)]
        binary_string = f"0b{'_'.join(chunks)}"

        # Index 'ABCDEFGH' korrekt zuordnen
        print(f"    {binary_string}, // Square {i} ({'ABCDEFGH'[i%8]}{i//8 + 1})")

    print("];")

if __name__ == "__main__":
    # Entpackt das Tupel mit beiden Masken-Listen
    rook_masks, bishop_masks = generate_premasks()

    print("--- ROOK MASKS ---")
    print_rust_array(rook_masks, "ROOK_RELEVANT_OCCUPANCY")

    print("\n--- BISHOP MASKS ---")
    print_rust_array(bishop_masks, "BISHOP_RELEVANT_OCCUPANCY")