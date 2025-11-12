a  = """
[
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
]
""".replace("[", "").replace("]", "").replace(",", "").strip().split("\n")
a = [int(x, 16) for x in a]

b  = """
[
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
]
""".replace("[", "").replace("]", "").replace(",", "").strip().split("\n")
b = [int(x, 16) for x in b]

def format_rust_array(data_list, array_name="GeneratedArray"):
    """
    Converts a list of integers/hex into a formatted Rust array 
    with binary representation and chess square comments.
    """
    print(f"pub const {array_name}: [u64; {len(data_list)}] = [")
    
    files = "ABCDEFGH"
    
    for i, val in enumerate(data_list):
        # 1. Convert to 64-bit binary string (MSB on left)
        bin_str = f"{val:064b}"
        
        # 2. Insert underscores every 8 bits for readability
        # Range 0 to 64 with step 8
        chunks = [bin_str[j:j+8] for j in range(0, 64, 8)]
        formatted_bin = "_".join(chunks)
        
        # 3. Calculate Square and Algebraic Notation for the comment
        # Assuming standard Little-Endian Rank-File mapping (A1=0, B1=1 ... H8=63)
        file_idx = i % 8
        rank_idx = i // 8
        
        if rank_idx < 8:
            algebraic = f"{files[file_idx]}{rank_idx + 1}"
        else:
            algebraic = "N/A" # Handle lists larger than 64 items gracefully
            
        # 4. Print the formatted line
        print(f"    0b{formatted_bin}, // Square {i} ({algebraic})")
        
    print("];")

# --- OPTION 1: Convert your specific hex list ---
my_hex_list = [
    0x8a80104000800020,
    0x140002000100040
]

# --- OPTION 2: Generate the actual King Attacks (to match your example) ---
def generate_king_attacks():
    king_moves = []
    for square in range(64):
        attacks = 0
        file = square % 8
        rank = square // 8
        
        # Iterate over all 8 neighbors
        for d_file in [-1, 0, 1]:
            for d_rank in [-1, 0, 1]:
                if d_file == 0 and d_rank == 0:
                    continue
                
                target_file = file + d_file
                target_rank = rank + d_rank
                
                if 0 <= target_file < 8 and 0 <= target_rank < 8:
                    target_square = target_rank * 8 + target_file
                    attacks |= (1 << target_square)
        king_moves.append(attacks)
    return king_moves

if __name__ == "__main__":
    format_rust_array(a, "MAGICS_ROOK")
    format_rust_array(b, "MAGICS_BISHOP")